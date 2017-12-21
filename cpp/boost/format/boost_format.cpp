#include <iostream>
#include <boost/format.hpp>

int main(){

  std::cout << boost::format("%s is a %s sample") % "this" % "format" << std::endl;

  return 0;
}
