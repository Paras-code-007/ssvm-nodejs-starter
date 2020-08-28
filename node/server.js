const { base64_encode, base64_decode, ceaser_7 } = require('../pkg/base64_lib.js');
const express = require('express');

const app = express();
const port = process.env.PORT||3000;

app.use(express.urlencoded({ extended: true }));
app.use(express.json())

// app.use('/public',express.static(__dirname + '/public'));
//not required

app.get('/',function (req,res,next) {
  res.sendFile(__dirname+ '/public/index.html')
})

app.post('/encode',function (req,res,next) {
  res.send(base64_encode(req.body.data))
})

app.post('/decode',function (req,res,next) {
  try {
    res.send(base64_decode(req.body.data))
  } catch (error) {
    res.send("error plz enter the encoded text not the normal text")
  }
})

app.get('/challenge',function (req,res,next) {
  res.sendFile(__dirname+ '/public/challenge.html')
})

app.post('/challenge',function (req,res,next) {
  res.send(ceaser_7(req.body.guess))
})

app.post('/guess',function (req,res,next) {
  if(req.body.guess=="ceaser cipher"){
    res.send("yes you are correct")
  }else{
    res.send("Wrong-- Try again")
  }
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))



/*
var bodyParser = require('body-parser')
app.use(bodyParser.urlencoded({
  extended: true
})); 
*/