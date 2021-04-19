import React from 'react'
import { Link } from 'react-router-dom'

import { ContractLink, ProfileLink, ProjectLink } from './Links'
import { mapProjectViewLimited } from './Helpers'

function ProjectCard (props) {
  const data = props.data
  const project = mapProjectViewLimited(data)
  const showContract = project.numContracts > 0
  return (
    <div className='card mb-3 bg-gray' style={{ width: '100%' }}>
      <div className='card-body'>
        <div className='card-title d-flex flex-row'>
          <h5 className='pe-3'>Project <ProjectLink projectName={project.name} /></h5>
          <div className='gray pe-3'><big>{project.numContracts}</big> contract(s)</div>
          {showContract &&
            <div className='gray pe-3'>last version <ContractLink contractHash={project.lastVersionContractHash} version={project.lastVersion} /></div>}
          {showContract &&
            <div className='gray'>by <ProfileLink userName={project.publisher} /></div>}
          <div className='ms-auto badge bg-success mt-1 mb-2'>{project.auditStatus}</div>
        </div>
        <div className='d-flex flex-row card-text'>
          <div className='flex-grow-1 d-flex flex-row'>
            <div className='card-text gray'>{project.description}</div>
          </div>
          {showContract &&
            <div className='ms-3 align-bottom '>
              <Link to={'/contract/' + project.lastVersionContractHash} className='btn btn-secondary'>Last deployed contract</Link>
            </div>}
          <div className='ms-3 align-bottom '>
            <Link to={'/projectInfo/' + project.name} className='btn btn-secondary'>Project details</Link>
          </div>
        </div>
      </div>
    </div>
  )
}

export { ProjectCard }
