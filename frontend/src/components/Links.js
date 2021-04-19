import React from 'react'
import { Link } from 'react-router-dom'

function ContractLink (props) {
  const contractHash = props.contractHash
  const version = props.version
  return <Link className='navigate' to={'/contract/' + contractHash}>{version}</Link>
}

function ProjectLink (props) {
  const projectName = props.projectName
  return <Link className='navigate' to={'/projectInfo/' + projectName}>{projectName}</Link>
}

function ProfileLink (props) {
  const userName = props.userName
  return <Link className='navigate' to={'/profileStats/' + userName}>{userName}</Link>
}

export { ContractLink, ProjectLink, ProfileLink }
