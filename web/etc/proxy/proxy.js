// Very simple CORS proxy

// based on: 
// https://medium.com/nodejsmadeeasy/a-simple-cors-proxy-for-javascript-applications-9b36a8d39c51

var express = require('express'),
    request = require('request'),
    bodyParser = require('body-parser'),
    app = express();

var limit = typeof(process.argv[2]) != 'undefined' ? process.argv[2] : '2mb';
console.log('Using limit: ', limit);

app.use(bodyParser.json({limit: limit}));

app.all('*', function (req, res, next) {
    res.header("Access-Control-Allow-Origin", "*");
    res.header("Access-Control-Allow-Headers", "*");
    res.header("Access-Control-Allow-Methods", "GET");

    if (req.method === 'OPTIONS') {
        res.send(); // CORS Preflight
        return;
    }
    if (req.path === '/favicon.ico') {
        return;
    }

    console.log(req.originalUrl);
    var target;
    if (req.originalUrl.startsWith('/https://') || req.originalUrl.startsWith('/http://')) {
        target = req.originalUrl.substring(1);
    } else {
        var prefix;
        if (req.originalUrl.startsWith('/unstable.sepolia.beacon-api.nimbus.team')) {
            // this one works only over http://
            prefix = 'http:/';
        } else {
            prefix = 'https:/';
        }
        target = prefix + req.originalUrl;
    }
    console.log('>>> ' + target);
    if (!target) {
        res.status(500).send({ error: 'target missing' });
        return;
    }

    request({ 
            url: target, 
            method: req.method, 
            json: req.body,
            headers: {} 
        },
        function (error, response, _) {
            if (error) {
                console.error('error: ' + error);
                return;
            }
            console.log('<<< ' + target + ': ' + response.statusCode);
        })
        .pipe(res);
});

app.set('port', process.env.PORT || 3000);

app.listen(app.get('port'), function () {
    console.log('Proxy server listening on port ' + app.get('port'));
});
