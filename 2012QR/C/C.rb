#!/usr/bin/env ruby
# -*- coding: utf-8 -*-
@@debug=false
def getis;gets.split.map(&:to_i);end;($fn=$*[0])&&$>=File::open($fn.chomp(".in")+".out","w");def each_case(nl=false)nc,
=getis;1.upto(nc){|c|print "Case ##{c}: ";nl&&puts;$fn&&STDOUT.puts([Time::now.strftime("%T"),c,?/,nc]*" ");yield c}end
def log(m)
  warn("[log]#{m}") if @@debug
end
def log2(t, m)
  warn("[log][#{t}]#{m}") if @@debug
end

def solve(a, b)
  log [a, b]
  visit = {}
  ans = 0
  (a..b).each do |i|
    # log i

    # if visit[i]==1
    #   next
    # end

    visit[i]=1
    is = i.to_s.split(//)
    is2 = is.dup
    
    is2.rotate!(-1)
    while is != is2
      i2 = is2.join("").to_i
      # visit[i2]=1
      # log2 "rotate", i2

      if a <= i2 && i2 <= b
        log [i, i2]

        if visit[i2] != 1
          ans += 1
        end
      end
      # log is2
      is2.rotate!(-1)
    end
  end
  ans
end

each_case{|i|
  line = getis
  a = line.shift
  b = line.shift
  
  puts solve(a, b)
}
