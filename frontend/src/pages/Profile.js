import React, { useState } from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { CertificateCard } from '../components/CertificateCard'
import { ProjectCard } from '../components/ProjectCard'
import { mapContract, loader, mapProjectViewLimited, getBgByStatus } from '../components/Helpers'

const tabs = {
  STATS: 'stats',
  PROJECTS: 'projects',
  AUDITS: 'audits'
}

function ProfilePageCommon (props, tab) {
  const { profileId } = useParams()
  const [showSubmitButton, setShowSubmitButton] = useState(true)
  const [newProjectName, setNewProjectName] = useState('')
  const [newProjectDesc, setNewProjectDesc] = useState('')
  const [formURL, setFormURL] = useState('')
  const [formSummary, setFormSummary] = useState('')
  const [formCommaSeparated, setFormCommaSeparated] = useState('')
  const [auditHash, setAuditHash] = useState('')
  const [auditRadioBasicValidity, setAuditRadioBasicValidity] = useState(undefined)
  const [auditRadioApprove, setAuditRadioApprove] = useState(undefined)
  const [auditRating, setAuditRating] = useState(undefined)

  async function submitNewProject (e) {
    e.preventDefault()
    setShowSubmitButton(false)
    try {
      await props._near.contract.register_project(
        {
          project_name: newProjectName,
          description: newProjectDesc,
          url: formURL,
          owners: formCommaSeparated.split(/[ ,]+/)
        }, '200000000000000', '1')
      // TODO
      window.location.href = '#/projectInfo/' + newProjectName
    } catch (e) {
      console.log('result', e)
      setShowSubmitButton(true)
    }
  }

  async function registerAsAuditor (e) {
    e.preventDefault()
    setShowSubmitButton(false)
    try {
      await props._near.contract.register_auditor(
        {
          user_id: profileId
        }, '200000000000000', '0')
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
          summary: formSummary,
          standards_confirmed: formCommaSeparated.split(/[ ,]+/),
          basic_validity_passed: auditRadioBasicValidity,
          contract_approved: auditRadioApprove,
          score: parseInt(auditRating)
        }, '200000000000000', '1')
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

  const fetchCertificates = async (...args) => {
    return await props._near.contract.get_auditor_certificates({ user_id: args[1] })
  }

  const fetchContract = async (...args) => {
    return mapContract(await props._near.contract.get_contract({ contract_hash: args[1] }))
  }

  const { data: user } = useSWR(['user_id', profileId], fetchUser, { errorRetryInterval: 500 })
  const { data: certificates } = useSWR(user && user.auditor !== null ? ['user_audits', profileId] : null, fetchCertificates, { errorRetryInterval: 500 })
  const { data: contract } = useSWR(auditHash ? ['contract', auditHash] : null, fetchContract, { errorRetryInterval: 500 })

  const userProjects = user && user.projects_owned.map((data, index) => {
    return <ProjectCard {...props} key={index} data={data} />
  })

  const userCertificates = certificates && certificates.length > 0 ? certificates.map((data, index) => {
    return <CertificateCard {...props} key={index} data={data} />
  }) : <div>No certificates found</div>

  const isMe = profileId === props.signedAccountId

  return props.connected ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight'>
            <h5>Profile</h5>
          </div>
          <div className='p-2 bd-highlight' />
          <div className='p-2 bd-highlight'>
            <h5 className='gray'>{profileId}</h5>
          </div>

          <div className='ms-auto bd-highlight' />
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
              {user && !user.auditor
                ? (
                  <div>
                    <h5>Not an auditor. Contact to devs to register as auditor</h5>
                    {/*
                    TODO
                    <h4>Not an auditor. Register?</h4>
                    <div className='p-2 bd-highlight' />
                    {showSubmitButton ? <button className='btn btn-primary' onClick={(e) => registerAsAuditor(e)}>Register as auditor</button> : loader()}
                    */}
                  </div>)
                : userCertificates}
            </div>
            {isMe && user && user.auditor &&
              <div className='mb-3 py-2'>
                <hr />
                <h4>Submit an audit</h4>
                <form onSubmit={(e) => submitNewAudit(e)}>
                  <small>// 5suuACmAzbTj8oyv4bQUjuJZbRinGMAKMLorDDEFzu4a - quick example</small>
                  <div className='d-flex align-items-center justify-content-center'>
                    <div className='form-group' style={{ width: '600px', margin: '25px' }}>
                      <label className='mt-3'>Contract hash</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: 55E7imniT2uuYrECn17qJAk9fLcwQW4ftNSwmCJL5Di' onChange={(e) => setAuditHash(e.target.value)}
                      />
                      <div className='mt-3' />
                      <div className='d-flex align-items-center justify-content-center'>
                        {!contract
                          ? <small className='me-auto'>Nothing found</small> : <small className='me-auto'>Found</small>}
                        {contract &&
                          <small className='gray me-3'>Project: {contract.project_name}</small>}
                        {contract &&
                          <small className='gray'>Version: {contract.version}</small>}
                      </div>

                      <div />

                      <div className='mt-3' />
                      <label>Report URL</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: github.com/near' onChange={(e) => setFormURL(e.target.value)}
                      />
                      <div className='mt-3' />
                      <label>Summary, 255 symbols max</label>
                      <textarea
                        rows='4' className='form-control mt-2'
                        placeholder='Example: Fifteen critical and eight high severity issues were found, along with recommendations on how to fix them. Additionally, some medium and lower severity issues were found and explained. Some changes were proposed to follow best practices and reduce the potential attack surface.' onChange={(e) => setFormSummary(e.target.value)}
                      />
                      <div className='mt-3' />
                      <label>Standards, comma-separated, optional</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: FOO-16, BAR-141' onChange={(e) => setFormCommaSeparated(e.target.value)}
                      />
                      <small className='gray'>Leave blank if no standards are confirmed</small>
                      <div className='mt-3' />
                      <label>Basic contract validity verdict</label>
                      <div className='mt-1' />
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault1' onChange={(e) => setAuditRadioBasicValidity(true)} />
                        <label className='form-check-label small'>I confirm the source code of the contract has been downloaded from the Blockchain and it has been compiled into <samp className='small'>{auditHash}</samp></label>
                      </div>
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault1' onChange={(e) => setAuditRadioBasicValidity(false)} />
                        <label className='form-check-label small'>The condition above is false</label>
                      </div>
                      <small className='gray'>This mark corresponds to basic contract validity knowledge at <u>contracts.one</u>. If you have audited NOT an open source code, please mark the condition above as false and go to the next question.</small>
                      <div className='mt-3' />
                      <label>General contract audit verdict</label>
                      <div className='mt-1' />
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault2' onChange={(e) => setAuditRadioApprove(true)} />
                        <label className='form-check-label small'>I confirm the source code of the contract is considered safe and can be used by anyone in any way without risks of potential funding losses</label>
                      </div>
                      <div className='form-check'>
                        <input className='form-check-input' type='radio' name='flexRadioDefault2' onChange={(e) => setAuditRadioApprove(false)} />
                        <label className='form-check-label small'>The condition above is false</label>
                      </div>
                      <small className='gray'>If the contract has dangerous flaws, please mark the condition above as false. Then the contract must be updated by the developer and uploaded under a new version. Please make sure that your report is reachable by URL and explains clearly the flaws.</small>
                      <div className='mt-3' />
                      <label>Overall contract quality rating, integer in range [1..10]</label>
                      <input
                        type='text' className='form-control mt-2'
                        placeholder='Example: 7' onChange={(e) => setAuditRating(e.target.value)}
                      />
                      <small className='gray'>Even in case of having dangerous flaws, the overall code quality may be very good. This score is mostly about your subjective understanding of how good the code is organized and structured, how easy to get into details, etc.</small>
                      <div className='mt-3' />
                      {showSubmitButton ? <button className='btn btn-outline-primary' disabled={!contract || !formURL || auditRadioBasicValidity === undefined || auditRadioApprove === undefined || !auditRating}>Submit an audit</button> : loader()}
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
