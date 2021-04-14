import React, { useState } from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { mapContract, loader } from '../components/Helpers'

const tabs = {
  STATS: 'summer',
  PROJECTS: 'winter',
  AUDITS: 'spring'
}

function ProfilePageCommon (props, tab) {
  const { profileId } = useParams()
  const [showSubmitButton, setShowSubmitButton] = useState(true)
  const [newProjectName, setNewProjectName] = useState('')
  const [newProjectDesc, setNewProjectDesc] = useState('')
  const [formURL, setFormURL] = useState('')
  const [formCommaSeparated, setFormCommaSeparated] = useState('')
  const [auditHash, setAuditHash] = useState('')
  const [auditRadioSafe, setAuditRadioSafe] = useState(undefined)
  const [auditRating, setAuditRating] = useState(undefined)

  async function submitNewProject (e) {
    e.preventDefault()
    console.log(formCommaSeparated)
    console.log(formCommaSeparated.split(/[ ,]+/))
    setShowSubmitButton(false)
    try {
      await props._near.contract.register_project(
        {
          project_name: newProjectName,
          description: newProjectDesc,
          url: formURL,
          owners: formCommaSeparated.split(/[ ,]+/)
        }, '200000000000000', '0')
      // TODO
      window.location.href = '#/projectInfo/' + newProjectName
    } catch (e) {
      console.log('result', e)
      setShowSubmitButton(true)
    }
  }

  async function submitNewAudit (e) {
    e.preventDefault()
    console.log(formCommaSeparated)
    console.log(formCommaSeparated.split(/[ ,]+/))
    console.log(auditRating)
    console.log(contract)
    setShowSubmitButton(false)
    try {
      await props._near.contract.sign_audit(
        {
          project_name: contract.project_name,
          version: contract.version,
          report_url: formURL,
          standards_confirmed: formCommaSeparated.split(/[ ,]+/),
          approved: auditRadioSafe,
          score: auditRating === undefined ? null : parseInt(auditRating)
        }, '200000000000000', '0')
      // TODO
      window.location.href = '#/contract/' + auditHash
    } catch (e) {
      console.log('result', e)
      setShowSubmitButton(true)
    }
  }

  const fetchUser = async (...args) => {
    return await props._near.contract.get_user({ user_id: args[1] })
  }

  const fetchUserAudits = async (...args) => {
    return await props._near.contract.get_user_audits({ user_id: args[1] })
  }

  const fetchContract = async (...args) => {
    return args[1] === '' ? mapContract(null) : mapContract(await props._near.contract.get_contract({ contract_hash: args[1] }))
  }

  const { data: user } = useSWR(['user_id', profileId], fetchUser, { errorRetryInterval: 500 })
  const { data: audits } = useSWR(['user_audits', profileId], fetchUserAudits, { errorRetryInterval: 500 })
  const { data: contract } = useSWR(['contract', auditHash], fetchContract, { errorRetryInterval: 500 })

  const userProjects = user && user.projects_owned.map((data, index) => {
    return (
      <div key={index} className='container g-0'>
        <div>
          <Link to={`/projectInfo/${data}`}>{data}</Link>
        </div>
      </div>
    )
  })

  const userAudits = audits && audits.map((data, index) => {
    return (
      <div key={index} className='container g-0'>
        <div>
          <Link to={`/projectInfo/${data.project_name}`}>{data.project_name}, v. {data.version}</Link>
        </div>
      </div>
    )
  })

  const isMe = profileId === props.signedAccountId

  return props.connected ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight my-gray'>
            <h5>Profile</h5>
          </div>
          <div className='p-2 bd-highlight' />
          <div className='p-2 bd-highlight'>
            <h5 className='gray'>{profileId}</h5>
          </div>

          <div className='px-4 bd-highlight' />
          {isMe
            ? <button className='btn btn-outline-secondary' onClick={() => props._near.logOut()}>Sign out</button>
            : <div />}
        </div>
        <hr />
      </div>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight'>
            {tab === tabs.PROJECTS
              ? <h5 className='nonavigate'>Projects</h5>
              : <h5><Link className='navigate' to={`/profileProjects/${profileId}`}>Projects</Link></h5>}
          </div>
          <div className='p-2 bd-highlight' />
          <div className='py-2 bd-highlight'>
            {tab === tabs.AUDITS
              ? <h5 className='nonavigate'>Audits</h5>
              : <h5><Link className='navigate' to={`/profileAudits/${profileId}`}>Audits</Link></h5>}
          </div>
          <div className='p-2 bd-highlight' />
          <div className='py-2 bd-highlight'>
            {tab === tabs.STATS
              ? <h5 className='nonavigate'>Stats</h5>
              : <h5><Link className='navigate' to={`/profileStats/${profileId}`}>Stats</Link></h5>}
          </div>
        </div>
      </div>
      <div className='container g-0 px-5'>
        {tab === tabs.PROJECTS &&
          <div>
            <div className='pb-3'>
              {userProjects}
            </div>
            <hr />
            {isMe &&
              <div className='mb-3 py-2'>
                <h4>Create a new project</h4>
                <form onSubmit={(e) => submitNewProject(e)}>
                  <div className='d-flex align-items-center justify-content-center'>
                    <div className='form-group' style={{ width: '600px', margin: '25px' }}>
                      <label className='mt-3'>Project name</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: My-awesome-project-3000' onChange={(e) => setNewProjectName(e.target.value)}
                      />
                      <label className='mt-3'>Project description (max 255 symbols)</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: Developer platform to create apps that put users back in control of their data and assets'
                        onChange={(e) => setNewProjectDesc(e.target.value)}
                      />
                      <label className='mt-3'>Project URL</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: github.com/near' onChange={(e) => setFormURL(e.target.value)}
                      />
                      <label className='mt-3'>Owners, comma-separated</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: root.near, alex.near' onChange={(e) => setFormCommaSeparated(e.target.value)}
                      />
                      {showSubmitButton ? <button className='btn btn-outline-primary mt-5' disabled={!newProjectName || !newProjectDesc || !formURL || !formCommaSeparated}>Create a new project</button> : loader()}
                    </div>
                  </div>
                </form>
              </div>}
          </div>}
        {tab === tabs.AUDITS &&
          <div>
            <div className='pb-3'>
              {userAudits}
            </div>
            <hr />
            {isMe &&
              <div className='mb-3 py-2'>
                <h4>Submit an audit</h4>
                <form onSubmit={(e) => submitNewAudit(e)}>
                  <div className='d-flex align-items-center justify-content-center'>
                    <div className='form-group' style={{ width: '600px', margin: '25px' }}>
                      <label className='mt-3'>Contract hash</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: 55E7imniT2uuYrECn17qJAk9fLcwQW4ftNSwmCJL5Di' onChange={(e) => setAuditHash(e.target.value)}
                      />
                      <div className='mt-3'>Project: {contract && contract.project_name ? contract.project_name : '(unknown)'}</div>
                      <div className='mt-1'>Version: {contract && contract.version ? contract.version : '(unknown)'}</div>
                      <label className='mt-3'>Report URL</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: github.com/near' onChange={(e) => setFormURL(e.target.value)}
                      />
                      <label className='mt-3'>Standards, comma-separated, optional</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: FOO-16, BAR-141' onChange={(e) => setFormCommaSeparated(e.target.value)}
                      />
                      <label className='mt-3 mb-2'>Audit verdict</label>
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault' onChange={(e) => setAuditRadioSafe(true)} />
                        <label className='form-check-label'>The contract is safe</label>
                      </div>
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault' onChange={(e) => setAuditRadioSafe(false)} />
                        <label className='form-check-label'>The contract is NOT safe</label>
                      </div>
                      <label className='mt-3'>Contract quality rating, integer in range [1..10], optional</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: 7' onChange={(e) => setAuditRating(e.target.value)}
                      />
                      {showSubmitButton ? <button className='btn btn-outline-primary mt-5' disabled={!contract || !formURL || auditRadioSafe === undefined}>Submit an audit</button> : loader()}
                    </div>
                  </div>
                </form>
              </div>}
          </div>}
      </div>
    </div>
  ) : loader()
}

function ProfileStatsPage (props) {
  return ProfilePageCommon(props, tabs.STATS)
}

function ProfileProjectsPage (props) {
  return ProfilePageCommon(props, tabs.PROJECTS)
}

function ProfileAuditsPage (props) {
  return ProfilePageCommon(props, tabs.AUDITS)
}

export { ProfileStatsPage, ProfileProjectsPage, ProfileAuditsPage }
