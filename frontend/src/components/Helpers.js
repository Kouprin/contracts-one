import React from 'react'

const NEAR = '\u24C3\u202F'

const fromNear = (s) => parseFloat(s) / 1e24 || 0

function loader () {
  return (
  // key='1' is needed by InfiniteScroll
    <div className='d-flex justify-content-center' key='1'>
      <div className='spinner-grow' role='status'>
        <span className='visually-hidden'>Loading...</span>
      </div>
    </div>
  )
}

const mapProjectView = (c) => {
  return c ? {
    id: c.project_id,
    name: c.project_name,
    description: c.description,
    url: c.url,
    owners: c.owners,
    contracts: c.contracts,
    lastVersion: c.last_version
  } : null
}

const mapCertificateView = (c) => {
  return c ? {
    projectName: c.project_name,
    version: c.version,
    author: c.author,
    reportUrl: c.report_url,
    summary: c.summary,
    standardsConfirmed: c.standards_confirmed,
    basicValidityPassed: c.basic_validity_passed,
    contractApproved: c.contract_approved,
    score: c.score
  } : null
}

const mapContract = (c) => {
  return c ? {
    audits: c.audits,
    certificates: c.certificates,
    commitHash: c.commit_hash,
    contractName: c.contract_name,
    hash: c.hash,
    projectName: c.project_name,
    publishedTime: c.published_time,
    publisher: c.publisher,
    safetyLevel: c.safety_report.safety_level,
    safetyIssues: c.safety_report.safety_issues,
    sourceCodeSize: c.source_code_size,
    standardsDeclared: c.standards_declared,
    version: c.version
  } : null
}

const mapProject = (c) => {
  return c ? {
    id: c.project_id,
    name: c.project_name,
    url: c.url,
    contracts: c.contracts,
    description: c.description,
    lastVersion: c.last_version,
    owners: c.owners
  } : null
}

export { NEAR, fromNear, loader, mapContract, mapProject, mapProjectView, mapCertificateView }
