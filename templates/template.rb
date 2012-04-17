#!/usr/bin/env ruby
# -*- coding: utf-8 -*-

def getis;gets.split.map(&:to_i);end;($fn=$*[0])&&$>=File::open($fn.chomp(".in")+".out","w");def each_case(nl=false)nc,
=getis;1.upto(nc){|c|print "Case ##{c}: ";nl&&puts;$fn&&STDOUT.puts([Time::now.strftime("%T"),c,?/,nc]*" ");yield}end
def log(m); warn(m); end

each_case{
  # gets
  # cs=getis
  # puts cs.reduce(:^)==0 ? cs.reduce(:+)-cs.min : "NO"
 
  
}
