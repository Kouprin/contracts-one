import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'
import Moment from 'react-moment'

import { mapProject, loader } from '../components/Helpers'

function ProjectInfoPage (props) {
  const { projectName } = useParams()

  const fetchProject = async (...args) => {
    return mapProject(await props._near.contract.get_project({ project_name: args[1] }))
  }

  const { data: project } = useSWR(['project', projectName], fetchProject, { errorRetryInterval: 500 })

  const owners = project && project.owners.map((data, index) => {
    return (
      <div key={index} className='container g-0'>
        <div>
          <Link to={`/profileProjects/${data}`}>{data}</Link>
        </div>
      </div>
    )
  })

  const versions = project && project.contracts.map((data, index) => {
    return (
      <div key={index} className='container g-0'>
        <div key={index} className='row'>
          <div className='col-1'>
            <Link to={`/contract/${data.hash}`}>{data.version}</Link>
          </div>
          <div className='col-4'>
            <samp className='small'><Link to={`/contract/${data.hash}`}>{data.hash}</Link></samp>
          </div>
          <div className='col-7'>
            Published <Moment unix fromNow>{data.published_time / 1000000000}</Moment> by <Link to={`/profileProjects/${data.publisher}`}>{data.publisher}</Link>
          </div>
        </div>
      </div>
    )
  })

  return props.connected && project ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight my-gray'>
            <h5>Project</h5>
          </div>
          <div className='p-2 bd-highlight' />
          <div className='p-2 bd-highlight'>
            <h5 className='gray'>{project.project_name}</h5>
          </div>
        </div>
        <div className='mb-3 py-2'>
          <h4>Description</h4>
          <div>{project.description}</div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Known versions</h4>
          <div>{versions}</div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Owners</h4>
          <div>{owners}</div>
        </div>
      </div>
    </div>
  ) : loader()
}

export default ProjectInfoPage
