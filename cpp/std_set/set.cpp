#include <iostream>
#include <set>

using namespace std;

int main(){

  set<int> st{1, 2, 3};
  cout << "size " << st.size() << endl;
  st.insert(4);
  cout << "size " << st.size() << endl;
  st.erase(2);
  cout << "size " << st.size() << endl;
  cout << "2 " << st.count(2) << endl;
  cout << "3 " << st.count(3) << endl;

  return 0;
}
