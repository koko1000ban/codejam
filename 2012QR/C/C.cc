// gcc
//args: -fopenmp
//flags: -fopenmp

#include <iostream>
#include <sstream>
#include <cstring>
#include <stdexcept>
#include <cstdarg>
// #include <thread>
#include <sys/time.h>
#include <cassert>
#include <math.h>

#ifdef _OPENMP
#include <omp.h>
#endif

#define rep(i,n)  REP(i,0,n)
#define REP(i,a,b) for(int i=(a);i<(b);++i)
#define len(a)  int((a).size())
#define all(x) (x).begin(), (x).end()
#define EXIST(s,e)  ((s).find(e)!=(s).end())
#define pb push_back
#define mp make_pair


using namespace std;
static bool on_debug;
template<class T> inline string toString(T x) {ostringstream sout;sout<<x;return sout.str();}

void debug2(const char* fmt, ...){
  if(!on_debug)return;
  
  va_list args;
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  va_end(args);
  fflush(stderr);
}

#include <deque>
#include <map>
#include <vector>
#include <algorithm>
#include <map>

// 回答

void init(){
  
  
  
}

class Solver
{
 private:
  int a;
  int b;
  string out_s_;
  
 public:
  Solver() : a(0),b(0),out_s_(""){};

  string out_s(){
    return out_s_;
  }
  
  //lock必須
  void input(){
    cin >> a;
    cin >> b;
  }

  int toi(vector<int> v){
    int num=0;
    int len=v.size();
    rep(i, len){
      // i | v[i] | v[i]*pow((double)10, (len-i-1))
      num += v[i]*pow((double)10, (len-i-1));
    }
    return num;
  }
  
  //処理を行って結果文字をout_sに格納
  void solve(){
    stringstream ss;

    map<int, int> visit;
    string as = toString(a);
    const int l = as.size();
    int ans=0;
    
    REP(i, a, b+1) {
      //  i
      if(visit[i]==1){continue;}
      
      visit[i]=1;
      vector<int> s(l);
      int num = i;
      int j = l-1;
      while(num>0){
        s[j--] = num%10;
        num /= 10;
      }
      // s

      int hit=0;
      rep(j, l-1){
        rotate(s.begin(), s.begin()+l-1, s.end());
        int si = toi(s);
        if(a <= si && si <= b){
          if(visit[si] != 1){
            // cout << i << " | " << si << endl;
            //  i | si
            hit++;
          }
        }
        visit[si]=1;
        if(si==i)break;
      }

      if(hit>0){
        rep(g, hit+1){
          ans += g;
        }
      }
    }
    
    ss << ans;
    out_s_=ss.str();
  }
  
};

void print_answer(int t, int T, string ans){
  cout << "Case #" << t+1 << ": " << ans << endl;
}

/*-----------------------------------------------*/

double sec(){
  struct timeval tv;
  gettimeofday(&tv, NULL);
  return tv.tv_sec + tv.tv_usec * 1e-6;
}

void print_pstatus(int t, int T, double d0, double d1, int nth){
  static const int L=5;
  if(T>L && t % (T/L) != 0) return;
  else if(t - (nth - 1) / 2 <= 0) fprintf(stderr, "[case: %d / %d]\n", t, T);
  else {
    double d = sec();
    fprintf(stderr, "[case: %d / %d | time: %.3f / %.3f ]\n", t, T, d-d0, (d1-d0) + (d-d1)/(t-(nth-1) / 2) * T);
  }
}

int main(int argc, char **argv) {
  //single or parallel
  bool parallel=true;
  on_debug=true;
  
  for(int i=1;i<argc;i++){
    if(strcmp(argv[i], "-p")==0) parallel=on_debug=true;
    else if(strcmp(argv[i], "-s")==0) on_debug=true;
    else{
      parallel=false;
    }
  }

  double d0 = sec();
  init();
  double d1 = sec();
  debug2("[init: %.3f]\n", d1-d0);
  
  int T;
  cin >> T;
  
  if(!parallel){
    debug2("[mode: single thread]\n");
    rep(t, T){
      if(on_debug)print_pstatus(t, T, d0, d1, 1);
      Solver solver;
      solver.input();
      solver.solve();
      print_answer(t, T, solver.out_s());
      fflush(stdout);
    }
    
  }else{
#ifdef _OPENMP

    const int nth =  omp_get_max_threads();
    debug2("[mode: parallel mode processor(%d) thread(%d)]\n", omp_get_num_procs(), nth);

    vector<string> out(T);
    vector<bool> done(T);
    
    int solved_t=0;
    omp_lock_t lock;
    omp_init_lock(&lock);
    
#pragma omp parallel for schedule(dynamic, 1)
    rep(t, T){
      Solver solver;

      //これで所定のインプットに他する答えをきちんとだせるのか?
      int my_t;
      omp_set_lock(&lock);
      {
        if(on_debug)print_pstatus(t, T, d0, d1, nth);
        my_t = solved_t++;
        solver.input();
      }
      omp_unset_lock(&lock);

      solver.solve();
      
      omp_set_lock(&lock);
      {
        out[my_t]=solver.out_s();
        done[my_t]=true;
      }
      omp_unset_lock(&lock);
    }

    omp_destroy_lock(&lock);
    
    rep(t, T){
      assert(done[t]);
      print_answer(t, T, out[t]);
    }
#else
    cerr << "error: compile with -fopenmp on gcc" << endl;
#endif
  }
  
  double d2=sec();
  debug2("[end: %.3f]\n", d2-d0);
  
  return 0;
}
