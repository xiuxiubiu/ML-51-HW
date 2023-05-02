<script>
	import * as echarts from "echarts";
	import { onMounted } from "vue";

	const { invoke } = window.__TAURI__.tauri;

	const colors = {
		0: "red",
		1: "blue",
		2: "gray",
		3: "green",
		4: "purple",
		5: "black",
		6: "orange",
		7: "brown",
	};

	var myChart;

	function initCharts(points) {
		var chartDom = document.getElementById("main");
		myChart && myChart.clear();
		myChart = echarts.init(chartDom);
		var option;

		option = {
			title: {
				text: "K-means clustering",
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
			],
		};

		option && myChart.setOption(option);
	}

	async function build() {
		let points = await invoke("rand_points", { num: 1000 });
		let clusters = await invoke("kmeans", {
			k: 7,
			points: points,
			max: 1000,
		});
		let scatterData = [];
		for (const k in clusters) {
			for (let i in clusters[k]) {
				scatterData.push({
					name: "point",
					value: [clusters[k][i][0], clusters[k][i][1]],
					itemStyle: { color: colors[k] },
				});
			}
		}
		initCharts(scatterData);
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
