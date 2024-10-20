const { type, name } = $arguments
const compatible_outbound = {
  tag: 'COMPATIBLE',
  type: 'direct',
}

let compatible
let config = JSON.parse($files[0])
let proxies = await produceArtifact({
  name: 'join',
  type: /^1$|col/i.test(type) ? 'collection' : 'subscription',
  platform: 'sing-box',
  produceType: 'internal',
})

config.outbounds.push(...proxies)

config.outbounds.map(i => {
  if (['all', 'all-auto'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies))
  }
  if (['Set', 'Tet'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /^(?!.*?流量).*Enet/i))
  }
  if (['Sfs', 'Tfs'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /^(?!.*?流量).*Fish/i))
  }
  if (['traffic-info'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /流量/i))
  }
  if (['ls'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /jia|nas|home|global/i))
  }
  if (['ly'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /ll|global/i))
  }
  if (['relayafter'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /global/i))
  }
  if (['relaybefore'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /^(?!.*?流量).*(Enet|Fish).*/i))
  }
  if (['ais'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /.*(Enet|Fish).*(美|日|新|台|德|法|芬|英|土)/i))
  }
  if (['games'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /.*(Enet|Fish).*(G|g)(A|a)(M|m)(E|e)/i))
  }
  if (['US'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /.*(Enet|Fish).*美/i))
  }
  if (['SG'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /.*(Enet|Fish).*新/i))
  }
  if (['EU'].includes(i.tag)) {
    i.outbounds.push(...getTags(proxies, /.*(Enet|Fish).*(德|法|芬|英|土)/i))
  }
})

config.outbounds.forEach(outbound => {
  if (outbound.tag === 'tjglobal' || outbound.tag === 'hyglobal') {
    outbound.detour = 'relaybefore';
  }
})

config.outbounds.forEach(outbound => {
  if (Array.isArray(outbound.outbounds) && outbound.outbounds.length === 0) {
    if (!compatible) {
      config.outbounds.push(compatible_outbound)
      compatible = true
    }
    outbound.outbounds.push(compatible_outbound.tag);
  }
});

$content = JSON.stringify(config, null, 2)

function getTags(proxies, regex) {
  return (regex ? proxies.filter(p => regex.test(p.tag)) : proxies).map(p => p.tag)
}

