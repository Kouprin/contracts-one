import React from 'react'
import { HashRouter as Router, Link, Route, Switch } from 'react-router-dom'
import 'error-polyfill'
import 'bootstrap/dist/js/bootstrap.bundle'
import 'bootstrap/dist/css/bootstrap.min.css'
import './App.scss'

import * as nearAPI from 'near-api-js'

import LandingPage from './pages/Landing'
import CertificatesPage from './pages/Certificates'
import ContractPage from './pages/Contract'
import ProjectsPage from './pages/Projects'
import ProjectInfoPage from './pages/ProjectInfo'
import { ProfileStatsPage, ProfileProjectsPage, ProfileAuditsPage } from './pages/Profile'
import Logo from './images/logo.png'
import Profile from './images/profile.png'

const IsMainnet = false // TODO window.location.hostname === 'contracts.one'
const TestNearConfig = {
  accountSuffix: 'testnet',
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  contractName: 'dev-1618917933127-5935675',
  walletUrl: 'https://wallet.testnet.near.org'
}
const MainNearConfig = {
  accountSuffix: 'near',
  networkId: 'mainnet',
  nodeUrl: 'https://rpc.mainnet.near.org',
  contractName: 'c.nearbet.near',
  walletUrl: 'https://wallet.near.org'
}

const NearConfig = IsMainnet ? MainNearConfig : TestNearConfig

class App extends React.Component {
  constructor (props) {
    super(props)

    this._near = {}

    this._near.lsKey = NearConfig.contractName + ':v01:'

    this._near.config = NearConfig

    this.state = {
      connected: false,
      account: null
    }

    this._initNear().then(() => {
      this.setState({
        signedIn: !!this._near.accountId,
        signedAccountId: this._near.accountId,
        connected: true
      })
    })
  }

  async _initNear () {
    const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    const near = await nearAPI.connect(Object.assign({ deps: { keyStore } }, NearConfig))
    this._near.keyStore = keyStore
    this._near.near = near

    this._near.walletConnection = new nearAPI.WalletConnection(near, NearConfig.contractName)
    this._near.accountId = this._near.walletConnection.getAccountId()

    this._near.account = this._near.walletConnection.account()
    this._near.contract = new nearAPI.Contract(this._near.account, NearConfig.contractName, {
      viewMethods: [
        'get_project',
        'get_contract',
        'get_contract_safety_report',
        'get_certificate',
        'get_all_projects',
        'get_project_last_version',
        'get_contract_source_code',
        'get_user',
        'get_auditor_certificates',
        'get_all_certificates'
      ],
      changeMethods: [
        'create_user',
        'register_project',
        'register_contract',
        'register_auditor',
        'sign_audit',
        'submit_audit_feedback'
      ]
    })

    this._near.logOut = () => {
      this._near.walletConnection.signOut()
      this._near.accountId = null
      this.setState({
        signedIn: !!this._accountId,
        signedAccountId: this._accountId
      })
    }

    this._near.refreshAllowance = async () => {
      alert("You're out of access key allowance. Need sign in again to refresh it")
      await this.logOut()
      await this.requestSignIn()
    }
  }

  async requestSignIn (e) {
    e && e.preventDefault()
    const appTitle = 'Contracts One'
    await this._near.walletConnection.requestSignIn(
      NearConfig.contractName,
      appTitle
    )
    return false
  }

  render () {
    const passProps = {
      _near: this._near,
      refreshAllowance: () => this._near.refreshAllowance(),
      ...this.state
    }
    const header = !this.state.connected ? (
      <div>Connecting... <span className='spinner-grow spinner-grow-sm' role='status' aria-hidden='true' /></div>
    ) : (this.state.signedIn ? (
      <div>
        <Link className='navbar-brand' to={`/profileProjects/${this.state.signedAccountId}`}>
          {this.state.signedAccountId}
          <img src={Profile} alt='Profile' className='px-2 d-inline-block align-middle' style={{ opacity: 0.85 }} />
        </Link>
      </div>
    ) : (
      <div>
        <button
          className='btn btn-primary'
          onClick={(e) => this.requestSignIn(e)}
        >Sign in with NEAR Wallet
        </button>
      </div>
    ))

    return (
      <div className='App text-white' style={{ backgroundColor: '#000000' }}>
        <Router basename={process.env.PUBLIC_URL}>
          <nav className='navbar navbar-expand-lg navbar-dark mb-3' style={{ backgroundColor: '#2F2F2F' }}>
            <div className='container-fluid'>
              <div style={{ marginLeft: '3%' }} />
              <Link className='navbar-brand' to='/' title='contracts.one'>
                <img src={Logo} alt='[BETA] Contracts One' className='d-inline-block align-middle' style={{ opacity: 1 }} />
              </Link>
              <div style={{ marginLeft: '1%' }} />
              <button
                className='navbar-toggler' type='button' data-bs-toggle='collapse'
                data-bs-target='#navbarSupportedContent' aria-controls='navbarSupportedContent'
                aria-expanded='false' aria-label='Toggle navigation'
              >
                <span className='navbar-toggler-icon' />
              </button>
              <div className='collapse navbar-collapse' id='navbarSupportedContent'>
                <ul className='navbar-nav me-auto mb-2 mb-lg-0'>
                  <li className='nav-item'>
                    <Link className='nav-link' aria-current='page' to='/projects'>Projects</Link>
                  </li>
                  <li className='nav-item'>
                    <Link className='nav-link' aria-current='page' to='/certificates'>Certificates</Link>
                  </li>
                  <li className='nav-item'>
                    <Link className='nav-link' aria-current='page' to='/issues'>Issues</Link>
                  </li>
                </ul>
                <form className='d-flex'>
                  {header}
                </form>
                <div style={{ marginRight: '3%' }} />
              </div>
            </div>
          </nav>

          <Switch>
            <Route exact path='/'>
              <LandingPage {...passProps} />
            </Route>
            <Route exact path='/projects'>
              <ProjectsPage {...passProps} />
            </Route>
            <Route exact path='/projects/:page'>
              <ProjectsPage {...passProps} />
            </Route>
            <Route exact path='/certificates'>
              <CertificatesPage {...passProps} />
            </Route>
            <Route exact path='/certificates/:page'>
              <CertificatesPage {...passProps} />
            </Route>
            <Route exact path='/projectInfo/:projectName'>
              <ProjectInfoPage {...passProps} />
            </Route>
            <Route exact path='/contract/:contractHash'>
              <ContractPage {...passProps} />
            </Route>
            <Route exact path='/profileStats/:profileId'>
              <ProfileStatsPage {...passProps} />
            </Route>
            <Route exact path='/profileProjects/:profileId'>
              <ProfileProjectsPage {...passProps} />
            </Route>
            <Route exact path='/profileAudits/:profileId'>
              <ProfileAuditsPage {...passProps} />
            </Route>
          </Switch>
        </Router>
      </div>
    )
  }
}

export default App
