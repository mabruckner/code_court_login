const request = require('request');
const spawnSync = require('child_process').spawnSync;
const electron = require('electron');
const ipc = require('electron').ipcMain;
const crypto = require('crypto');
const fs = require('fs');
const app = electron.app;

const BrowserWindow = electron.BrowserWindow

const path = require('path')
const url = require('url')

let token

let mainWindow

let config

function loadConfig() {
    config = JSON.parse(fs.readFileSync('config.json', {encoding: 'utf8'}));
}

loadConfig();

request({
    method: 'POST',
    uri: url.resolve(config.server, '/api/login'),
    json: {
        email: config.admin.email,
        password: config.admin.password
    }
}, function(error, response, body) {
    token = body['access_token'];
    console.log("token: "+token);
    if(error || !token) {
        console.log(error);
        console.log(response);
        console.log(body);
    }
})

function makePassword() {
    var length = 8;
    //var guard = new RegExp('[A-HJ-NP-Za-kmnp-z2-9]');
    var guard = new RegExp('[A-HJ-NP-Z2-9]');
    var pass = '';
    while (pass.length < length) {
        var val = String.fromCodePoint(crypto.randomBytes(1)[0]);
        if(guard.test(val)){
            pass += val;
        }
    }
    return pass;
}

ipc.on('get-config', function(evt, data) {
    evt.sender.send('config', config);
});
ipc.on('create-user', function (evt, user) {
    var pass = makePassword();
    request({
        method: 'POST',
        uri: url.resolve(config.server, '/api/make-defendant-user'),
        headers: {
            'Authorization': 'Bearer '+token
        },
        json: {
            name: user.name,
            username: user.username,
            email: user.email,
            contest_name: user.bracket,
            password: pass
        }
    }, function(error, response, body) {
        console.log(error);
        console.log(response);
        console.log(body);
        if(body && body['status'] == 'Success') {
            request({
                method: 'POST',
                uri: url.resolve(config.server, '/api/update-user-metadata'),
                headers: {
                    'Authorization': 'Bearer '+token
                },
                json: {
                    user_email: user.email,
                    misc_metadata: {
                        signin: Math.floor(Date.now()/1000),
                        teacher: user.teacher
                    }
                }
            }, function(error, response, body) {
                console.log(body);
            });

            evt.sender.send('create-success', body);
            var cargo = spawnSync('cargo', ['run', '--', config.printer.path, config.server, user.name, user.email, pass], {cwd: path.join(__dirname, 'rust')});
            console.log(cargo.stdout.toString());
            console.log(cargo.stderr.toString());
            evt.sender.send('finish-print','');
        } else {
            evt.sender.send('create-error', {"error": error, "body": body});
        }
        console.log('password: '+pass);
    });
})

function createWindow () {
    mainWindow = new BrowserWindow({width:100, height:100, frame:false, kiosk: true})

    mainWindow.loadURL(url.format({
        pathname: path.join(__dirname, 'index.html'),
        protocol: 'file:',
        slashes: true
    }))

    mainWindow.on('closed', function () {
        mainWindow = null
    })
    //electron.Menu.setApplicationMenu(new electron.Menu())
}

app.on('ready', createWindow)

app.on('window-all-closed', function () {
    app.quit()
})

console.log('Hello World');
