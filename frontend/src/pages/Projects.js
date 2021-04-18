import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { loader, getBgByStatus, mapProjectViewLimited } from '../components/Helpers'

const FetchLimit = 25

function ProjectsPage (props) {
  let { page } = useParams()
  if (page === undefined) {
    page = 0
  }
  const from = Math.max(0, page - 1) * FetchLimit

  const fetchProjects = async (...args) => {
    return await props._near.contract.get_all_projects({ from: args[1], to: args[1] + FetchLimit })
  }

  const { data: projects } = useSWR(['all_projects', from], fetchProjects, { errorRetryInterval: 500 })

  const projectList = projects && projects.map((data, index) => {
    const project = mapProjectViewLimited(data)
    const projectInfoDestination = '/projectInfo/' + project.name
    const versionDestination = project.lastVersion && '/contract/' + project.lastVersionContractHash
    return (
      <div key={index} className='row'>
        <div className='col-3' style={{ minWidth: '300px' }}>
          <div className='d-flex flex-row'>
            <Link to={projectInfoDestination}>{project.name}</Link>
            <div className={'mt-1 mx-2 badge bg-success ' + getBgByStatus(project.auditStatus)}>
              {project.auditStatus}
            </div>
          </div>
        </div>
        {project.lastVersion &&
          <div className='col-1'>
            <Link to={versionDestination}>{project.lastVersion}</Link>
          </div>}
      </div>
    )
  })

  return projects ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <h4 className='py-2 bd-highlight'>
            Projects
          </h4>
        </div>
      </div>
      <div className='container g-0 px-5'>
        <div className='mb-3 py-2'>
          {projectList}
        </div>
      </div>
    </div>
  ) : loader()
}

export default ProjectsPage
