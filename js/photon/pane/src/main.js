const electron = require('electron');
const app = electron.app;
const BrowserWindow = electron.BrowserWindow;

const path = require('path');
const url = require('url');

let mainwindow;

function createWindow(){

  mainwindow = new BrowserWindow({height : 500,
                                  width : 600,
                                  frame : false});

  mainwindow.loadURL(url.format({
    pathname : path.join(__dirname, 'index.html'),
    protocol : 'file',
    slashes : true
  }));

mainwindow.on('closed', function(){
  mainwindow = null;
});
}

app.on('ready', createWindow);

app.on('window-all-closed', function(){
  if(process.platform !== 'darwin'){
    app.quit();
  }
});

app.on('activate', function(){
  if(mainwindow == null){
    createWindow();
  }
});
