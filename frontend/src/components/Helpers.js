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
    name: c.project_name
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

export { NEAR, fromNear, loader, getBgByStatus, mapContract, mapProject, mapProjectViewLimited }
