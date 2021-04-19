import React from 'react'

import { ProfileLink, ProjectLink } from '../components/Links'
import { mapCertificateView } from '../components/Helpers'

function CertificateCard (props) {
  const data = props.data
  const certificate = mapCertificateView(data)
  const standards = certificate.standardsConfirmed.map((data, index) => {
    return (
      <div key={index} className='badge bg-success mx-1'>
        {data}
      </div>
    )
  })
  return (
    <div className='card mb-3 bg-gray' style={{ width: '100%' }}>
      <div className='card-body'>
        <div className='d-flex flex-row'>
          <h5 className='card-title pe-3'>Certificated <ProjectLink projectName={certificate.projectName} /></h5>
          <div className='card-title gray pe-3'>version {certificate.version}</div>
          <div className='card-title gray pe-3'>by <ProfileLink userName={certificate.author} /></div>
          <div className='me-auto card-title'>{standards}</div>
          <big className='pe-3 card-title'><small className='gray'>score:</small> {certificate.score}</big>
          <big className='card-title'><small className='gray'>verdict:</small> {certificate.approved ? 'approved' : 'refused'}</big>
        </div>
        <div className='d-flex flex-row card-text'>
          <div className='w-75'>
            <div className='card-text gray'>{certificate.summary}</div>
          </div>
          <div className='ms-auto'>
            <a href={'//' + certificate.reportUrl} className='btn btn-secondary'>Go to report</a>
          </div>
        </div>
      </div>
    </div>
  )
}

export { CertificateCard }
