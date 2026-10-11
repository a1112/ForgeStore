import hashlib
def decode(data):
    pos=0
    def value():
        nonlocal pos
        c=data[pos:pos+1];assert c
        if c==b'i':
            pos+=1;end=data.index(b'e',pos);raw=data[pos:end];assert raw and raw not in [b'-0'] and (raw==b'0' or not raw.startswith(b'0')) and not raw.startswith(b'-0');pos=end+1;return int(raw)
        if c==b'l':
            pos+=1;items=[]
            while data[pos:pos+1]!=b'e':items.append(value())
            pos+=1;return items
        if c==b'd':
            pos+=1;items={};last=None
            while data[pos:pos+1]!=b'e':
                k=value();assert isinstance(k,bytes) and (last is None or last<k) and k not in items;last=k;items[k]=value()
            pos+=1;return items
        assert c.isdigit();end=data.index(b':',pos);raw=data[pos:end];assert raw==b'0' or not raw.startswith(b'0');n=int(raw);pos=end+1;v=data[pos:pos+n];assert len(v)==n;pos+=n;return v
    result=value();assert pos==len(data);return result
def encode(v):
    if isinstance(v,int):return b'i'+str(v).encode()+b'e'
    if isinstance(v,bytes):return str(len(v)).encode()+b':'+v
    if isinstance(v,list):return b'l'+b''.join(encode(x) for x in v)+b'e'
    assert isinstance(v,dict);return b'd'+b''.join(encode(k)+encode(v[k]) for k in sorted(v))+b'e'
def verify(torrent,payload,name,comment):
    meta=decode(torrent);assert encode(meta)==torrent;info=meta[b'info'];assert info[b'name']==name.encode('utf-8') and info[b'length']==len(payload) and b'files' not in info
    length=info[b'piece length'];assert length==32768
    expected=b''.join(hashlib.sha1(payload[i:i+length]).digest() for i in range(0,len(payload),length));assert info[b'pieces']==expected and len(expected)==20
    assert meta[b'comment']==comment.encode() and b'announce' not in meta and b'announce-list' not in meta
    return {'payloadBytes':len(payload),'pieceLength':length,'pieceCount':1,'allPieceHashesMatchIndependentPayloadSha1':True,'infoHashSha1':hashlib.sha1(encode(info)).hexdigest(),'comment':comment,'infoNameUtf8':name,'createdBy':meta.get(b'created by',b'').decode(),'trackerless':True,'bencodeCanonicalAndFullyParsed':True}
