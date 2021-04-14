import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'
import Moment from 'react-moment'

import { loader, mapContract, mapProject } from '../components/Helpers'

function ContractPage (props) {
  const { contractHash } = useParams()

  const fetchProject = async (...args) => {
    return mapProject(await props._near.contract.get_project({ project_name: args[1] }))
  }

  const fetchContract = async (...args) => {
    return args[1] === '' ? mapContract(null) : mapContract(await props._near.contract.get_contract({ contract_hash: args[1] }))
  }

  const { data: contract } = useSWR(['contract_hash', contractHash], fetchContract, { errorRetryInterval: 500 })
  const { data: project } = useSWR(['project_name', contract ? contract.project_name : ''], fetchProject, { errorRetryInterval: 500 })

  const certificates = contract && contract.certificates.length ? contract.certificates.map((data, index) => {
    const approvedMsg = data.approved ? 'approved' : 'refused'
    return (
      <div key={index} className='container g-0'>
        <div>
          {approvedMsg} by <Link to={`/profileAudits/${data.author}`}>{data.author}</Link>
        </div>
      </div>
    )
  }) : <div>No certificates found</div>

  const standardsMap = new Map()
  contract && contract.certificates.forEach(certificate => {
    certificate.standards_confirmed.forEach(standard => {
      standardsMap.set(standard, (standardsMap.get(standard) || 0) + 1)
    })
  })

  const standards = standardsMap.size ? Array.from(standardsMap).map((data, index) => {
    return (
      <div key={index} className='container g-0 pt-2'>
        <div className='d-flex flex-row'>
          <div className='px-2 bg-success badge'>
            {data[0]}<small className='ps-2'>x{data[1]}</small>
          </div>
        </div>
      </div>
    )
  }) : <div>No standards confirmed</div>

  return props.connected && contract && project ? (
    <div className='pb-3'>
      <div className='container g-0 px-5'>
        <div className='d-flex flex-row bd-highlight mb-3'>
          <div className='py-2 bd-highlight my-gray'>
            <h5>Contract</h5>
          </div>
          <div className='p-2 bd-highlight' />
          <div className='p-2 bd-highlight'>
            <h5 className='gray'>{contract.hash}</h5>
          </div>
          <div className='px-5 bd-highlight' />
          <div className='bd-highlight'>
            <h5 className='btn-lg btn-outline disabled'>Safety score</h5>
          </div>
          <div className='bd-highlight'>
            <div className='btn btn-lg btn-outline-warning'>69</div>
          </div>
          <div className='bd-highlight'>
            <h5 className='btn-lg btn-outline-warning disabled'>Moderate</h5>
          </div>
        </div>
        <div className='mb-3 py-2'>
          <h4>Project</h4>
          <Link to={`/projectInfo/${contract.project_name}`}>{contract.project_name}</Link>
        </div>
        <div className='mb-3 py-2'>
          <h4>Description</h4>
          <div>{project.description}</div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Contract details</h4>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Contract name:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.contract_name}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Version:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.version}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Publisher:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.publisher}
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Published time:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <Moment unix fromNow>{contract.published_time / 1000000000}</Moment>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Hash:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              <samp className='small'>{contract.hash}</samp>
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Basic contract validity:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              3 / 5 confirmations received
            </div>
          </div>
          <div className='row'>
            <div className='col-2' style={{ minWidth: '200px' }}>
            Source code:
            </div>
            <div className='col-4' style={{ minWidth: '200px' }}>
              {contract.source_code_size} bytes <Link to='#'>(download)</Link>
            </div>
          </div>
        </div>
        <hr />
        <div className='mb-3 py-2'>
          <h4>Standards</h4>
          <div>{standards}</div>
        </div>
        <div className='mb-3 py-2'>
          <h4>Certificates</h4>
          <div>{certificates}</div>
        </div>
        <hr />
        <div className='small'>
          <div className='pb-2'>Reporting of basic contract validity</div>
          <div className='gray'>To check basic contract validity, you asked to download and compile the source code from the link above.</div>
          <div className='gray'>Please use <Link to='/rules'>Contracts One CLI Developer Tools</Link> or do it manually as the following.</div>
          <div className='gray pb-1'>To compile the source code, run the following lines in the terminal:</div>
          <div className='small px-2 py-1 bg-gray'>
            <div>
              <samp className='mt-5'>RUSTFLAGS='-C link-arg=-s' cargo +nightly build --target wasm32-unknown-unknown --release</samp>
            </div>
            <div>
              <samp className='mt-5'>cd target/wasm32-unknown-unknown/release && near dev-deploy contracts_one.wasm</samp>
            </div>
          </div>
          <div className='gray pt-2 pb-2'>Grab your freshly generated <samp>dev-*</samp> account from stdout and execute the following:</div>
          <div className='small px-2 py-1 bg-gray'>
            <div>
              <samp className='mt-5'>near state [dev-account] --networkId=testnet --nodeUrl=https://rpc.testnet.near.org</samp>
            </div>
          </div>
          <div className='gray pt-2'>Check that <samp className='small'>code_hash</samp> is equal to <samp className='small'>{contract.hash}</samp>.</div>
          <div className='gray pt-2'>Go to the <Link to='/rules'>Basic Contract Validity Guide</Link> for more details.</div>
        </div>
        {false &&
          <div><button className='mt-3 btn btn-outline-success'>Confirm basic contract validity</button>
            <button className='mt-3 mx-2 btn btn-outline-danger'>Report basic contract validity abuse</button>
          </div>}
      </div>
    </div>
  ) : loader()
}

export default ContractPage
