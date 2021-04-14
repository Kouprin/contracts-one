import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { loader } from '../components/Helpers'

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

  const projectList = projects ? projects.map((data, index) => {
    const projectInfoDestination = '/projectInfo/' + data[0]
    const versionDestination = data[1] && '/contract/' + data[1][1]
    return (
      <div key={index} className='row'>
        <div className='col-2' style={{ minWidth: '200px' }}>
          <Link to={projectInfoDestination}>{data[0]}</Link>
        </div>
        {data[1] &&
          <div className='col-2' style={{ minWidth: '200px' }}>
            {data[1][0]}
          </div>}
        {data[1] &&
          <div className='col-4' style={{ minWidth: '200px' }}>
            <Link to={versionDestination}>{data[1][1]}</Link>
          </div>}
      </div>
    )
  }) : <div>No standards confirmed</div>

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
  ) : (loader())
}

export default ProjectsPage
