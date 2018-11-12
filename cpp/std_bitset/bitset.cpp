#include <iostream>
#include <bitset>
using namespace std;

int main(int argc, char const* argv[])
{
  bitset<10> bset;
  cout << bset << endl;
  cout << bset.size() << endl;
  bset.set(3);
  cout << bset << endl;
  cout << bset[3] << endl;
  cout << bset[4] << endl;
  bset.set();
  cout << bset << endl;
  cout << ~bset << endl;
  return 0;
}
