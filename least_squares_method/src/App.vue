<script>
	import * as echarts from "echarts";
	import { onMounted } from "vue";

	const { invoke } = window.__TAURI__.tauri;

	var myChart;

	function initCharts(points, line) {
		var chartDom = document.getElementById("main");
		if (myChart) {
			myChart.clear();
		}
		myChart = echarts.init(chartDom);

		var option;
		option = {
			title: {
				text: "Least Squares Method",
				left: "center",
			},
			xAxis: { min: -100, max: 100 },
			yAxis: { min: -100, max: 100 },
			series: [
				{
					symbolSize: 10,
					data: points,
					type: "scatter",
				},
				{
					data: line,
					type: "line",
					symbol: "none",
				},
			],
		};

		option && myChart.setOption(option);
	}

	async function build() {
		let points = await invoke("rand_points");
		let line = await invoke("get_line", {
			points: points,
		});
		initCharts(points, line);
	}

	export default {
		setup() {
			onMounted(build);
		},
		methods: {
			rebuild() {
				build();
			},
		},
	};
</script>

<template>
	<div id="main" style="float: right; width: 800px; height: 800px"></div>
	<div><button @click="rebuild">Rebuild</button></div>
</template>
