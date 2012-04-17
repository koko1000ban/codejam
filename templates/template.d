#! /usr/bin/env dmd -run

import std.stdio, std.string, std.conv;
import std.algorithm, std.array, std.bigint, std.math, std.range;
import std.datetime;
import core.thread;
import std.parallelism;

void log(Args ...)(string fmt, Args args){ stderr.writef(fmt~"\n", args); }

//	Input
string[] tokens;
int tokenId = 0;
string readToken() { for (; tokenId == tokens.length; ) tokens = readln.split, tokenId = 0; return tokens[tokenId++]; }
int readInt() { return to!int(readToken); }
long readLong() { return to!long(readToken); }

void prepare() {
}

class Solver{
  int caseNo;
  string answer_s;
  
  string line;
  
  this(ulong caseNo){
    this.caseNo = to!int(caseNo);
  }
  
  void input(){
    line = readln.chomp;
  }
  
  void solve(){
    Thread.sleep( dur!("msecs")(  1 ) );
    answer_s = "hoge";
  }

  string answer(){
    return answer_s;
  }
  
}

void print_pstatus(ulong t, int T, StopWatch sw, int nth){
  static const int L=5;
  if(T>L && t % (T/L) != 0) return;
  else if(t - (nth - 1) / 2 <= 0) log("[case: %d / %d]", t, T);
  else {
    sw.stop();
    log("[case: %d / %d | time: %.3d]", t, T, sw.peek().msecs);
  }
}

void main(string[] args) {

  // parallel mode or single mode
  auto parallel_mode = true;

  StopWatch sw;
  sw.start();
  
  // loop t and fetch input
  auto t = readInt;
  prepare;

  // solver initialize
  auto solvers = new Solver[t];
  foreach(ix, ref solver; solvers){
    solver = new Solver(ix+1);
    solver.input();
  }
  
  // parallel or single sovle
  if (parallel_mode) {
    log("[mode: parallel start cpus(%d)]", totalCPUs);

    foreach(ix, ref solver; parallel(solvers)) {
      print_pstatus(ix, t, sw, 1);
      solver.solve();
    }
    
  } else {
    log("[mode: single start]");
    
    foreach(int ix, ref solver; solvers){
      print_pstatus(ix, t, sw, 1);
      solver.solve();
    }
  }

  
  // output answer
  foreach(int ix, ref solver; solvers){
    writefln("Case #%d: %s", ix+1, solver.answer());
  }
  
  sw.stop();
  log("[finish time: %d[ms]]",  sw.peek().msecs);
}
