let txs = seeds.map(function(x){
  return invokeScript({
  dApp: dappAddress,
  call:{
    function:"withdraw",
    args: [
      { type:"Integer", value: amount}
      ]},
      payment: []
  }, x)
})
