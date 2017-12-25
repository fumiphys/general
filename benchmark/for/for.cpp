#include <iostream>
#include <time.h>

int main(){

  long sum = 0;
  int N = 1000000;
  int i;
  clock_t start = clock();
  for(i = 0; i < N; i++){
    sum += i;
  }
  clock_t end = clock();

  std::cout << "C++ " << sum << " " << (double)(end-start)/CLOCKS_PER_SEC << "sec." << std::endl;
}
