#include <iostream>
#include <time.h>

long fib(int n){
  if(n == 0)return 0;
  if(n == 1)return 1;
  return fib(n-1) + fib(n-2);
}

int main(){

  int N = 40;
  clock_t start = clock();
  long fibonacci = fib(N);
  clock_t end = clock();

  std::cout << "C++ " << fibonacci << " " << (double)(end-start)/CLOCKS_PER_SEC << "sec." << std::endl;

  return 0;
}
