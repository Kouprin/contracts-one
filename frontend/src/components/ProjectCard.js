import React from 'react'
import { Link } from 'react-router-dom'

import { ContractLink, ProfileLink, ProjectLink } from './Links'
import { mapProjectView } from './Helpers'

function ProjectCard (props) {
  const data = props.data
  const project = mapProjectView(data)
  let lastContract = null
  if (project.contracts) {
    project.contracts.forEach(element => {
      if (element.version === project.lastVersion) {
        lastContract = element
      }
    })
  }
  return (
    <div className='card mb-3 bg-gray' style={{ width: '100%' }}>
      <div className='card-body'>
        <div className='card-title d-flex flex-row'>
          <h5 className='pe-3'>Project <ProjectLink projectName={project.name} /></h5>
          {lastContract &&
            <div className='gray pe-3'><big>{project.contracts.length}</big> contract(s)</div>}
          {lastContract &&
            <div className='gray pe-3'>last version <ContractLink contractHash={lastContract.hash} version={project.lastVersion} /></div>}
          {lastContract &&
            <div className='gray'>by <ProfileLink userName={lastContract.publisher} /></div>}
          {/* lastContract &&
            <div className='ms-auto badge bg-success mt-1 mb-2'>test</div> */}
        </div>
        <div className='d-flex flex-row card-text'>
          <div className='flex-grow-1 d-flex flex-row'>
            <div className='card-text gray'>{project.description}</div>
          </div>
          {lastContract &&
            <div className='ms-3 align-bottom '>
              <Link to={'/contract/' + lastContract.hash} className='btn btn-primary'>Last deployed contract</Link>
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
