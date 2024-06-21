import { init, RuntimeMetadata, Methods } from 'merkleized-metadata'
import * as defaults from './defaults';
import { useState, useMemo, useId, useEffect } from 'react'

export default function App() {
  const [mm, setMm] = useState<Methods>()

  useEffect(() => {
    init().then((mm) => setMm(mm));
  }, [])

  return mm
    ? <Main mm={mm} />
    : <div className="loading">Loading...</div>
}

type MainArgs = {
  mm: Methods
}

function Main(args: MainArgs) {
  const mm = args.mm;
  const [metadata, setMetadata] = useState(defaults.METADATA)
  const [tx, setTx] = useState(defaults.TX)
  const [decimals, setDecimals] = useState(defaults.DECIMALS)
  const [tokenSymbol, setTokenSymbol] = useState(defaults.TOKEN_SYMBOL)
  const [specVersion, setSpecVersion] = useState(defaults.SPEC_VERSION)
  const [specName, setSpecName] = useState(defaults.SPEC_NAME)
  const [base58Prefix, setBase58Prefix] = useState(defaults.BASE58_PREFIX)
  const [additionalSigned, setAdditionalSigned] = useState(defaults.TX_ADDITIONAL_SIGNED)

  const runtimeMetadata = useMemo(() => {
    try {
      return RuntimeMetadata.fromHex(metadata);
    } catch (e) {
      return "Error decoding metadata:" + String(e)
    }
  }, [metadata])

  const digestHash = useMemo(() => {
    if (typeof runtimeMetadata === "string") {
      return "Invalid runtime metadata"
    }

    try {
      const digest = mm.generateMetadataDigest(runtimeMetadata, {
        base58Prefix,
        decimals,
        specName,
        specVersion,
        tokenSymbol
      })

      return digest.hash()
    } catch (e) {
      return "Error computing metadata hash: " + String(e)
    }
  }, [runtimeMetadata, base58Prefix, decimals, specName, specVersion, tokenSymbol])

  const extrinsicProof = useMemo(() => {
    if (typeof runtimeMetadata === "string") {
      return "Invalid runtime metadata"
    }

    try {
      return mm.generateProofForExtrinsic(tx, additionalSigned || undefined, runtimeMetadata).encode()
    } catch (e) {
      return "Error computing extrinsic proof: " + String(e)
    }
  }, [runtimeMetadata, tx, additionalSigned])

  return (
    <main>
      <h1>Merkleized Metadata Example</h1>
      <table>
        <tbody>
          <Row
            name="Decimals"
            value={(id) => <input id={id} type='number' value={decimals} onChange={(e) => setDecimals(parseInt(e.target.value) || 0)}/>}
          />
          <Row
            name="Token Symbol"
            value={(id) => <input id={id} value={tokenSymbol} onChange={(e) => setTokenSymbol(e.target.value)}/>}
          />
          <Row
            name="Spec Version"
            value={(id) => <input id={id} value={specVersion} onChange={(e) => setSpecVersion(parseInt(e.target.value) || 0)}/>}
          />
          <Row
            name="Spec Name"
            value={(id) => <input id={id} value={specName} onChange={(e) => setSpecName(e.target.value)}/>}
          />
          <Row
            name="Base58 Prefix"
            value={(id) => <input id={id} value={base58Prefix} onChange={(e) => setBase58Prefix(parseInt(e.target.value) || 0)}/>}
          />
          <Row
            name="Metadata Hex"
            value={(id) => <textarea id={id} value={metadata} onChange={(e) => setMetadata(e.target.value)}/>}
          />
          <Row
            name="Transaction Hex"
            value={(id) => <textarea id={id} value={tx} onChange={(e) => setTx(e.target.value)}/>}
          />
          <Row
            name="Additional Signed Hex"
            value={(id) => <textarea id={id} value={additionalSigned} onChange={(e) => setAdditionalSigned(e.target.value)}/>}
          />
          <Row
            name="Metadata Hash"
            value={(id) => digestHash}
          />
          <Row
            name="Extrinsic Proof"
            value={(id) => extrinsicProof}
          />
        </tbody>
      </table>
    </main>
  );
}

type RowArgs = {
  name: string,
  value: (id: string) => any
}

function Row(args: RowArgs) {
  const id = useId()

  return (
    <tr>
      <td className="key"><label htmlFor={id}>{args.name}</label></td>
      <td className="value">{args.value(id)}</td>
    </tr>
  )
}