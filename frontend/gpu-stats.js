let str = "";


function init() {
  var chart = new SmoothieChart({ minValue:0, grps: 30, millisPerPixel: 20, grid: { strokeStyle: '#555555', lineWidth: 1, millisPerLine: 1000, verticalSections: 4}, tooltip: true});
  canvas = document.getElementById('smoothie-chart');
  series = new TimeSeries();

  chart.addTimeSeries(series, {lineWidth:2,strokeStyle:'#00ff00'});
  chart.streamTo(canvas, 500);

  let socket = new WebSocket("ws://localhost:8120/ws/");

  socket.onmessage = (msg) => {
    // let tmp = msg.data
    // for (var i = 0; i < tmp.length; ++i) {
    //   var code = tmp.charCodeAt(i);
    //   console.log(`Byte: ${code}, Char: ${String.fromCharCode(code)}`)
    // }
    // console.log(Number(tmp))
    // socket.close()

    data = msg.data.replace(/[^0-9.]/g,'');
    series.append(Date.now(), data);
  }
}
