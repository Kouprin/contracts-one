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

function getBgByStatus (status) {
  if (status === 'Audited') return 'bg-success'
  return 'bg-danger'
}

const mapProjectViewLimited = (c) => {
  return c ? {
    auditStatus: c.audit_status,
    description: c.description,
    id: c.project_id,
    lastVersion: c.last_version,
    lastVersionContractHash: c.last_version_contract_hash,
    name: c.project_name,
    publisher: c.publisher,
    numContracts: c.num_contracts
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
    approved: c.approved,
    score: c.score
  } : null
}

// TODO
const mapContract = (c) => {
  return c || {}
}

// TODO
const mapProject = (c) => {
  return c || null
}

export { NEAR, fromNear, loader, getBgByStatus, mapContract, mapProject, mapProjectViewLimited, mapCertificateView }
