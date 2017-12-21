var page = require('webpage').create();

page.open('http://www.google.com', function(status){
  var title = page.evaluate(function(){
    return document.title;
  });
  console.log("status : "+status);
  console.log("title : "+title);
  phantom.exit();
});
