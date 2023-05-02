<script>
	import * as echarts from "echarts";
	import { onMounted } from "vue";

	const { invoke } = window.__TAURI__.tauri;

	var myChart;
	var points = [];

	function initCharts() {
		var chartDom = document.getElementById("main");
		myChart = echarts.init(chartDom);
		var option;

		option = {
			title: {
				text: "K Nearest Neighbors",
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
		points = await invoke("rand_points", { num: 10 });
		initCharts();
	}

	export default {
		setup() {
			onMounted(build);
		},
		data() {
			return {
				x: 0.0,
				y: 0.0,
			};
		},
		methods: {
			rebuild() {
				build();
			},
			async addPoint() {
				let color = await invoke("knn", { k: 3, sample: [this.x, this.y] });
				points.push({
					value: [this.x, this.y],
					itemStyle: {
						color: color,
					},
				});
				initCharts();

				this.x = 0.0;
				this.y = 0.0;
			},
		},
	};
</script>

<template>
	<div id="main" style="float: right; width: 800px; height: 800px"></div>
	<div><button @click="rebuild">Rebuild</button></div>
	<form @submit.prevent="addPoint">
		<span>x: </span
		><input type="number" step="0.01" min="-100" max="100" v-model="x" />
		<span>y: </span
		><input type="number" step="0.01" min="-100" max="100" v-model="y" />
		<button>Add Sample</button>
	</form>
</template>
