import React, { useState } from 'react'
import { mapContract } from '../components/Helpers'
import useSWR from 'swr'

function LandingPage (props) {
  const [query, setQuery] = useState('')

  async function go (e) {
    e.preventDefault()
    window.location.href = '#/contract/' + query
  }

  const fetchContract = async (...args) => {
    return mapContract(await props._near.contract.get_contract({ contract_hash: args[1] }))
  }

  const { data: contract } = useSWR(query ? ['contract', query] : null, fetchContract, { errorRetryInterval: 500 })

  return (
    <div
      className='container my-auto'
    >
      <div style={{ margin: '5%' }} />
      <h4 className='mb-5 text-center'>Welcome to contracts.one!</h4>
      <h5 className='text-center'>The decentralized source of truth for NEAR contracts</h5>
      <div style={{ margin: '10%' }} />
      <form onSubmit={(e) => go(e)}>
        <div className='d-flex align-items-center justify-content-center'>
          <div className='form-group' style={{ width: '600px', margin: '25px' }}>
            <label className='mt-3'>Put here contract hash or certificate id</label>
            <input
              type='text' className='form-control mt-2'
              placeholder='Example: 55E7imniT2uuYrECn17qJAk9fLcwQW4ftNSwmCJL5Di' onChange={(e) => setQuery(e.target.value)}
            />
            <small className='gray'>try <samp className='small'>5suuACmAzbTj8oyv4bQUjuJZbRinGMAKMLorDDEFzu4a</samp></small>
            <div style={{ margin: '10%' }} />
            <div className='d-flex justify-content-center'>
              <button className={'btn-lg btn btn-outline-primary ' + (contract ? '' : 'disabled')}>Go!</button>
            </div>
          </div>
        </div>
      </form>
    </div>
  )
}

export default LandingPage
