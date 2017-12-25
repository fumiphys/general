#include <iostream>
#include <string>

template <typename T>
class Three{
  public:
    void addElement(T);
    T getElement(int);
    Three();
    ~Three();
  private:
    T *num;
    int count;
};

template <typename T>
Three<T>::Three(){
  count = 0;
  num = new T[3];
}

template <typename T>
Three<T>::~Three(){
  delete[] num;
}

template <typename T>
void Three<T>::addElement(T elm){
  if(count == 3){
    std::cout << "count is 3" << std::endl;
  }
  else{
    num[count] = elm;
    count++;
  }
}

template <typename T>
T Three<T>::getElement(int i){
  if(i < 0 && i > 3){
    std::cout << "index out of range" << std::endl;
  }else{
    return num[i];
  }
}

int main(){

  Three<int> int_three;
  int_three.addElement(3);
  int_three.addElement(4);
  int_three.addElement(5);
  std::cout << int_three.getElement(2) << std::endl;
  int_three.addElement(6);

  Three<std::string> str_three;
  str_three.addElement(std::string("a"));
  str_three.addElement(std::string("b"));
  str_three.addElement(std::string("c"));
  std::cout << str_three.getElement(2) << std::endl;
  str_three.addElement(std::string("d"));

  return 0;
}
