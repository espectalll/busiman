'use strict';

$(() => {
	$('#power').submit((e) => {
		var $form = $(this);
		var data = $form.serialize();
		var req = $.post({
			url: "/power",
			data: data
		}).done((res) => {
			console.log(res);
		});
		e.preventDefault();
		e.unbind();
	});
});
