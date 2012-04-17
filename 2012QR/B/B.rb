#!/usr/bin/env ruby
# -*- coding: utf-8 -*-
@@debug=false
def getis;gets.split.map(&:to_i);end;($fn=$*[0])&&$>=File::open($fn.chomp(".in")+".out","w");def each_case(nl=false)nc,
=getis;1.upto(nc){|c|print "Case ##{c}: ";nl&&puts;$fn&&STDOUT.puts([Time::now.strftime("%T"),c,?/,nc]*" ");yield c}end
def log(m)
  warn("[log]#{m}") if @@debug
end

each_case{|i|
  line = getis
  n = line.shift
  s = line.shift
  p = line.shift
  gglrs = line
  
  log "\ndebug start : #{i}"
  log([n, s, p, gglrs])

  answer = 0
  gglrs.each do |ggl|
    log "-- -- take #{ggl} rest s:#{s}"
    
    # 0 1 のときは
    if ggl == 0 or ggl == 1
      if ggl >= p
        log "ans++ when 0 1 shortcut"
        answer+=1
      end
      next
    end

    avg = ggl / 3
    if avg >= p
      log "ans++ #{ggl} 's avg[#{avg}] >= #{p}"
      answer += 1
      next
    end
    
    mod = ggl % 3
    log "avg:#{avg} mod:#{mod}"
    case mod
    when 0
      if avg + 1 >= p && s > 0
        log "ans++ #{ggl} when 0 and use sup"
        s -= 1
        answer += 1
        next
      end
    when 1
      if avg + 1 >= p
        answer += 1
        log "ans++ #{ggl} when 1 and avg+1 > p"
        next
      end
    when 2
      if avg + 1 >= p
        log "ans++ #{ggl} when 2 and avg+1 > p"
        answer += 1
        next
      elsif s > 0 && avg + 2 >= p
        log "ans++ #{ggl} when 2 and use sup"
        s -= 1
        answer += 1
        next
      end
    end
  end

  log "answer is #{answer}"
  puts answer
}

