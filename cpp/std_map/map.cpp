#include <iostream>
#include <string>
#include <map>

void one(int x){
  std::cout << x << std::endl;
}

void two(int x){
  std::cout << -1 * x << std::endl;
}

int main(){

  std::map<std::string, void(*)(int)> com;
  com["one"] = one;
  com["two"] = two;

  com["one"](3);
  com["two"](4);

  return 0;
}
