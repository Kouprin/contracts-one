import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'

import { CertificateCard } from '../components/CertificateCard'
import { loader } from '../components/Helpers'

const FetchLimit = 25

function CertificatesPage (props) {
  let { page } = useParams()
  if (page === undefined) {
    page = 0
  }
  const from = Math.max(0, page - 1) * FetchLimit

  const fetchCertificates = async (...args) => {
    return await props._near.contract.get_all_certificates({ from: args[1], to: args[1] + FetchLimit })
  }

  const { data: certificates } = useSWR(['all_certificates', from], fetchCertificates, { errorRetryInterval: 500 })

  const certificatesList = certificates && certificates.map((data, index) => {
    return <CertificateCard {...props} key={index} data={data} />
  })

  return certificates ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <h4 className='py-2 bd-highlight'>
            All Certificates
          </h4>
          <div className='ms-auto bd-highlight' />
          <Link className={'btn btn-outline-secondary ' + (!props.signedIn ? 'disabled' : '')} to={'/profileAudits/' + props.signedAccountId}>Submit new audit</Link>
          <div className='px-2 bd-highlight' />
          <Link className={'btn btn-outline-secondary ' + (!props.signedIn ? 'disabled' : '')} to={'/profileAudits/' + props.signedAccountId}>My audits</Link>
          <div className='px-2 bd-highlight' />
          <Link className={'btn btn-outline-primary disabled ' + (!props.signedIn ? 'disabled' : '')} to={'/profileAudits/' + props.signedAccountId}>Request an audit</Link>
        </div>
      </div>
      <div className='container g-0 px-5'>
        <div className='mb-3 py-2'>
          {certificatesList}
        </div>
      </div>
    </div>
  ) : loader()
}

export default CertificatesPage
