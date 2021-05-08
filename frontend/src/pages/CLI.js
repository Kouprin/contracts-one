import React from 'react'
import { useParams } from 'react-router'
import { Link } from 'react-router-dom'
import useSWR from 'swr'
import Moment from 'react-moment'

import { loader, getBgByStatus } from '../components/Helpers'

function CLIPage (props) {
  return props.connected ? (
    <div className='pb-3'>
      <div className='container g-0 px-5 pt-3'>
        <div className='col-8'>
          <h4>Contracts.one CLI Developer and Auditor tools</h4>
          <div className='gray'>Contracts.one CLI is developed to improve and simplify experience of using the service. The tools are mostly about receiving source code of the contract and uploading it in a proper way.</div>
          <a href='https://github.com/Kouprin/contracts-one/tree/master/cli' className='mt-3 btn btn-outline-secondary'>CLI tools at GitHub</a>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Getting started</h5>
          <div className='gray pb-1'>Use yarn and node.js to build and execute.</div>
          <div className='gray'>At BETA stage, the CLI is not available at npmjs, so you have to build it locally.</div>
          <div className='small px-2 py-1 my-1 bg-gray'>
            <div>
              <samp className='mt-5'>yarn</samp>
            </div>
            <div>
              <samp className='mt-5'>./index.js</samp>
            </div>
          </div>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Getting packed source code of the contract</h5>
          <div className='gray'>Run</div>
          <div className='small px-2 py-1 my-1 bg-gray'>
            <div>
              <samp className='mt-5'>./index.js near source &lt;code-hash&gt; &lt;filename&gt;</samp>
            </div>
          </div>
          <div className='gray'>For example,</div>
          <div className='small px-2 py-1 my-1 bg-gray'>
            <div>
              <samp className='mt-5'>./index.js near source 8RcunxYGCt2RhNBnLeAJg1h7aYKbqi9MNHncZdJn8HqS test.bs58</samp>
            </div>
          </div>
          <div className='gray'>will create the file <samp className='small'>test.bs58</samp> with the proper data.</div>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Check the contract hash validity</h5>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Unpack the contract code</h5>
          <div className='gray'>Run</div>
          <div className='small px-2 py-1 my-1 bg-gray'>
            <div>
              <samp className='mt-5'>./index.js near unpack &lt;filename&gt;</samp>
            </div>
          </div>
          <div className='gray'>For example,</div>
          <div className='small px-2 py-1 my-1 bg-gray'>
            <div>
              <samp className='mt-5'>./index.js near test.bs58</samp>
            </div>
          </div>
          <div className='gray'>will extract the data from <samp className='small'>test.bs58</samp>.</div>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Publish the contract</h5>
        </div>
        <hr style={{ marginTop: '2%', marginBottom: '2%' }} />
        <div className='col-8'>
          <h5>Pack the source code of the contract</h5>
        </div>
      </div>
    </div>
  ) : loader()
}

export default CLIPage
