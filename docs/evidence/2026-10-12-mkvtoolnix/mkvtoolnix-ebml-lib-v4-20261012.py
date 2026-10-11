import struct,hashlib,io,wave
def vint(raw,pos,ident=False):
 assert pos<len(raw) and raw[pos]!=0
 mask=128;length=1
 while not raw[pos]&mask:mask>>=1;length+=1
 assert length<=8 and pos+length<=len(raw)
 value=int.from_bytes(raw[pos:pos+length],'big')
 if not ident:value &= (1<<(7*length))-1
 return value,pos+length,(not ident and value==(1<<(7*length))-1)
def elements(raw,start,end):
 pos=start;result=[]
 while pos<end:
  ident,pos,_=vint(raw,pos,True);size,pos,unknown=vint(raw,pos)
  if unknown:assert ident==0x18538067;size=end-pos
  assert pos+size<=end
  result.append((ident,pos,pos+size));pos+=size
 assert pos==end
 return result
def one(raw,seq,ident):
 found=[raw[a:b] for n,a,b in seq if n==ident];assert len(found)==1,(hex(ident),len(found));return found[0]
def uint(raw):return int.from_bytes(raw,'big')
def float_value(raw):
 assert len(raw) in (4,8);return struct.unpack('>f' if len(raw)==4 else '>d',raw)[0]
def verify_mka(raw,wav,title):
 top=elements(raw,0,len(raw));assert top[0][0]==0x1A45DFA3
 segments=[(a,b) for n,a,b in top if n==0x18538067];assert len(segments)==1
 contents=elements(raw,*segments[0]);info=one(raw,contents,0x1549A966);ie=elements(info,0,len(info))
 assert one(info,ie,0x7BA9).decode('utf-8')==title
 duration=float_value(one(info,ie,0x4489));scale=uint(one(info,ie,0x2AD7B1));assert abs(duration*scale/1e9-0.1)<1/8000
 tracks=one(raw,contents,0x1654AE6B);te=elements(tracks,0,len(tracks));entry=one(tracks,te,0xAE);ee=elements(entry,0,len(entry))
 assert uint(one(entry,ee,0xD7))==1 and uint(one(entry,ee,0x83))==2
 assert one(entry,ee,0x86)==b'A_PCM/INT/LIT'
 audio=one(entry,ee,0xE1);ae=elements(audio,0,len(audio))
 freq=[audio[a:b] for n,a,b in ae if n==0xB5];channels=[audio[a:b] for n,a,b in ae if n==0x9F];assert len(freq)<=1 and len(channels)<=1
 assert (float_value(freq[0]) if freq else 8000.0)==8000 and (uint(channels[0]) if channels else 1)==1 and uint(one(audio,ae,0x6264))==16
 payloads=[];blocks=[]
 for n,a,b in contents:
  if n!=0x1F43B675:continue
  for bn,ba,bb in elements(raw,a,b):
   if bn==0xA0:
    block=one(raw,elements(raw,ba,bb),0xA1)
   elif bn==0xA3:block=raw[ba:bb]
   else:continue
   track,p,_=vint(block,0);assert track==1 and p+3<=len(block)
   timecode=struct.unpack('>h',block[p:p+2])[0];flags=block[p+2];p+=3
   lacing=flags&6;assert lacing in (0,4),('Lacing outside this finite parser',flags)
   count=1
   if lacing==4:
    assert p<len(block);count=block[p]+1;p+=1;assert (len(block)-p)%count==0
   payloads.append(block[p:]);blocks.append({'relativeTimecode':timecode,'payloadBytes':len(block)-p,'lacing':'fixed' if lacing==4 else 'none','laceFrames':count})
 assert payloads
 with wave.open(io.BytesIO(wav),'rb') as rd:
  assert rd.getparams()[:4]==(1,2,8000,800);pcm=rd.readframes(800)
 assert b''.join(payloads)==pcm and len(pcm)==1600
 return {'title':title,'durationSeconds':duration*scale/1e9,'timestampScaleNs':scale,'durationTicks':duration,'nominalPcmDurationSeconds':0.1,'durationToleranceSeconds':1/8000,'codecId':'A_PCM/INT/LIT','rateHz':8000,'channels':1,'bitsPerSample':16,'samples':800,'pcmSizeBytes':1600,'pcmSha256':hashlib.sha256(pcm).hexdigest(),'pcmExactMatch':True,'samplingFrequencyUsesSpecificationDefault':not bool(freq),'channelsUsesSpecificationDefault':not bool(channels),'blocks':blocks,'fullMatroskaSpecificationVerified':False}
