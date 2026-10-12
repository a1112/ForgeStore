set encoding utf8
set terminal windows size 640,480
set xrange [-0.25:3.25]
set yrange [-1:10]
unset key
set title "MARKET BEFORE"
plot "C:/GnuplotTests/市场数据.dat" using 1:2 with linespoints lw 2 pt 7 ps 1.5 lc rgb "#0000FF"
