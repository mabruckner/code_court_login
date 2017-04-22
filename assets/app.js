import React from 'react';
import ReactDOM from 'react-dom';
const ipc = require('electron').ipcRenderer;

console.log(document.getElementById('root'));


class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = {page:'start', name:'', email:'', bracket:'', teacher:'', username:''};

        this.toStart=this.toStart.bind(this);
        this.toForm=this.toForm.bind(this);
        this.toLoad=this.toLoad.bind(this);
        this.toPrint=this.toPrint.bind(this);
        this.handleChange=this.handleChange.bind(this);
        ipc.on('create-error', function(evt, dat) {
            if(dat.body) {
                this.setState({error: dat.body.error});
            } else {
                this.setState({error: dat.error});
            }
            this.toForm(evt);
        }.bind(this));
        ipc.on('create-success', function(evt, dat) {
            this.toPrint(evt);
        }.bind(this));
        ipc.on('finish-print', function(evt, dat) {
            setTimeout(this.toStart, 10000, {});
        }.bind(this));
    }
    toStart(e) {
        this.setState({page:'start', name:'', email:'', bracket:'', teacher:'', username:'', error:null});
    }
    toForm(e) {
        this.setState({page:'form'});
    }
    toLoad(e) {
        ipc.send('create-user', {
            name: this.state.name,
            username: this.state.username,
            email: this.state.email+'@'+this.props.domain,
            bracket: this.state.bracket,
            teacher: this.state.teacher,
        });
        this.setState({page:'loading'});
    }
    toPrint(e) {
        this.setState({page:'print'});
    }
    handleChange(e) {
        const target = e.target;
        const name = target.name;

        if(name == 'bracket') {
            this.setState({bracket:target.value});
        } else if(name=='username') {
            this.setState({username:target.value});
        } else if(name=='email') {
            this.setState({email:target.value});
        } else if(name=='name') {
            this.setState({name:target.value});
        } else if(name=='teacher') {
            this.setState({teacher:target.value});
        }
    }
    render() {
        if(this.state.page == 'start'){
            return (
                <div className="hero-body">
                <div className="container has-text-centered">
                    <h1 className="title is-1">{this.props.strings.title}</h1>
                    <h4 className="subtitle is-4">{this.props.strings.subtitle}</h4>
                    <a className="button is-info is-large" onClick={this.toForm}>{this.props.strings.start}</a>
                </div>
                </div>);
        } else if(this.state.page == 'form'){
            const bracketOptions = this.props.brackets.map((bracket) => <option key={bracket}>{bracket}</option>);
            const tlist = this.props.teachers[this.state.bracket];
            let teacherOptions;
            if(tlist) {
                teacherOptions = <select name="teacher" value={this.state.teacher} onChange={this.handleChange}>
                    <option value="" disabled>Teacher</option>
                    {tlist.map((teacher) => <option key={teacher}>{teacher}</option>)}
                </select>;
            } else if(this.state.bracket=="") {
                teacherOptions = <span className="select"><select disabled><option>Select a Bracket</option></select></span>;
            } else {
                teacherOptions = <input name="teacher" value={this.state.teacher} className="input" type="text" placeholder="Teacher" onChange={this.handleChange}/>;
            }
            let message;
            if(this.state.error) {
                message = <article className="message is-danger">
                        <div className="message-header">
                            <p>Error</p>
                        </div>
                        <div className="message-body">
                            {this.state.error}
                        </div>
                    </article>;
            }
            console.log(bracketOptions);
            return (
                <div className="hero-body">
                <div className="infobox card">
                <header className="card-header">
                    <p className="card-header-title">{this.props.strings.info}</p>
                </header>
                <div className="card-content">
                    {message}
                    <div className="field">
                        <p className="control">
                            <input name="name" value={this.state.name} className="input" type="text" placeholder="Name" onChange={this.handleChange}/>
                        </p>
                    </div>
                    <div className="field">
                        <p className="control">
                            <input name="username" value={this.state.username} className="input" type="text" placeholder="Username" onChange={this.handleChange}/>
                        </p>
                    </div>
                    <div className="field is-grouped">
                        <p className="control is-expanded is-level">
                            <input className="input level-item" name="email" type="text" value={this.state.email} placeholder="Email" onChange={this.handleChange}/>
                        </p>
                        <p className="control level-item">@{this.props.domain}</p>
                    </div>
                    <div className="field">
                        <p className="control">
                            <span className="select">
                                <select name="bracket" value={this.state.bracket} onChange={this.handleChange}>
                                    <option value="" disabled>Bracket</option>
                                    {bracketOptions}
                                </select>
                            </span>
                        </p>
                    </div>
                    <div className="field">
                        <p className="control">
                            {teacherOptions}
                        </p>
                    </div>
                    <a className="button is-info" disabled={this.state.name==''||this.state.bracket==''||this.state.username==''||this.state.email==''} onClick={this.toLoad}>{this.props.strings.submit}</a>
                </div>
                </div>
                </div>);
        } else if(this.state.page=='loading') {
            return (
                <div className="hero-body">
                    <div className="container has-text-centered">
                        <h2 className="title is-2">{this.props.strings.wait}</h2>
                        <figure className="infobox image">
                            <img src="assets/loading.svg"/>
                        </figure>
                    </div>
                </div>);
        } else if(this.state.page=='print') {
            return (
                <div className="hero-body">
                    <div className="container has-text-centered">
                        <h2 className="title is-2">{this.props.strings.print}</h2>
                        <figure className="infobox image">
                            <img src="assets/printer.svg"/>
                        </figure>
                    </div>
                </div>);
        } else {
            return <p>oops</p>;
        }
    }
}

let config;

const brackets=['1400', '1620', 'open'];
const teachers={
    "1400": ["alpha", "beta"],
    "1620": ["beta", "gamma"],
    "open": ["gamma", "delta", "sigma", "none"],
};

ipc.send('get-config', {});
ipc.on('config', function (evt, data) {
    config = data;
    console.log(config);
    ReactDOM.render(
        <Main brackets={config.kiosk.brackets} teachers={config.kiosk.teachers} strings={config.kiosk.strings} domain={config.kiosk.domain} />,
        document.getElementById('root')
    );
});
