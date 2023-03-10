var express = require("express"),
    app = express.createServer();
app.use(express.static(__dirname'/styles'));
app.listen(3001);
