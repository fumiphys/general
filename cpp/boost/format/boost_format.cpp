#include <iostream>
#include <boost/format.hpp>
#include <string>

int main(){

  std::cout << boost::format("%s is a %s sample") % "this" % "format" << std::endl;
  std::cout << boost::format("%2% is a %1% sample") % "this" % "format" << std::endl;

  std::string format_str = (boost::format("%s is %s") % "this" % "boost/format").str();
  std::cout << format_str << std::endl;

  return 0;
}
