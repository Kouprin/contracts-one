import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { ProjectCard } from '../components/ProjectCard'
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

  const projectList = projects && projects.map((data, index) => {
    return <ProjectCard {...props} key={index} data={data} />
  })

  return projects ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <h4 className='py-2 bd-highlight'>
            All Projects
          </h4>
          <div className='ms-auto bd-highlight' />
          <Link className={'btn btn-outline-secondary ' + (!props.signedIn ? 'disabled' : '')} to={'/profileProjects/' + props.signedAccountId}>Create new project</Link>
          <div className='px-2 bd-highlight' />
          <Link className={'btn btn-outline-secondary ' + (!props.signedIn ? 'disabled' : '')} to={'/profileProjects/' + props.signedAccountId}>My projects</Link>
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
