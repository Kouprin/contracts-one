import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'
import Moment from 'react-moment'

import { loader, mapContract, mapProject } from '../components/Helpers'

function ContractPage (props) {
  const { contractHash } = useParams()

  const fetchProject = async (...args) => {
    return mapProject(await props._near.contract.get_project({ project_name: args[1] }))
  }

  const fetchContract = async (...args) => {
    return args[1] === '' ? mapContract(null) : mapContract(await props._near.contract.get_contract({ contract_hash: args[1] }))
  }

  const { data: contract } = useSWR(contractHash ? ['contract', contractHash] : null, fetchContract, { errorRetryInterval: 500 })
  const { data: project } = useSWR(contract ? ['project', contract.projectName] : null, fetchProject, { errorRetryInterval: 500 })

  const certificates = contract && contract.certificates.length ? contract.certificates.map((data, index) => {
    const approvedMsg = data.approved ? 'approved' : 'refused'
    return (
      <div key={index} className='container g-0'>
        <div>
          {approvedMsg} by <Link to={`/profileAudits/${data.author}`}>{data.author}</Link>
        </div>
      </div>
    )
  }) : <div>No certificates found</div>

  const standards = contract && contract.standardsDeclared.length ? contract.standardsDeclared.map((data, index) => {
    return (
      <div key={index} className='badge bg-primary me-2'>
        <small>{data}</small>
      </div>
    )
  }) : <div>No standards declared</div>

  const safety = contract && (contract.safetyLevel === 'High' ? 'bg-success' : (contract.safetyLevel === 'Moderate' ? 'bg-warning' : 'bg-danger'))

  const audits = contract && contract.audits.length ? contract.audits.map((data, index) => {
    return (
      <div key={index} className='badge bg-secondary me-2'>
        <small>{data}</small>
      </div>
    )
  }) : <div>No audits</div>

  const issues = contract && contract.safetyIssues.length ? contract.safetyIssues.map((data, index) => {
    return (
      <div key={index} className='container g-0'>
        <small>
          {data}
        </small>
      </div>
    )
  }) : <div>No issues found</div>

  return props.connected && contract && project ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight'>
            <h5>Contract</h5>
          </div>
          <div className='p-2 bd-highlight' />
          <div className='p-2 bd-highlight'>
            <h5 className='gray'>{contract.hash}</h5>
          </div>
        </div>
        <div className='mb-3 py-2'>
          <h4>Project</h4>
          <Link to={`/projectInfo/${contract.projectName}`}>{contract.projectName}</Link>
        </div>
        <div className='mb-3 py-2'>
          <h4>Description</h4>
          <div>{project.description}</div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Contract details</h4>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Contract name:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.contractName}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Version:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.version}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Publisher:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <Link to={`/profileProjects/${contract.publisher}`}>{contract.publisher}</Link>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Published time:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <Moment unix fromNow>{contract.publishedTime / 1000000000}</Moment>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Hash:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <samp className='small'>{contract.hash}</samp>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Audits:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {audits}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Source code:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.sourceCodeSize} bytes <Link to='/cli'>(download)</Link>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Safety report:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <div className={'badge ' + safety}>{contract.safetyLevel}</div>
              {issues}
            </div>
          </div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Standards</h4>
          {standards}
        </div>
        <div className='mb-3 py-2'>
          <h4>Certificates</h4>
          {certificates}
        </div>
      </div>
    </div>
  ) : loader()
}

export default ContractPage
