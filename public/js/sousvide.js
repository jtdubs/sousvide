function refresh() {
  $.getJSON("/rest/state", function (data) {
    if (data["set_temp"]) {
      $("#set_temp_label").html(data["set_temp"] + "&#x2109;");
    } else {
      $("#set_temp_label").html("--");
    }
    if (data["cur_temp"]) {
      $("#cur_temp_label").html(data["cur_temp"] + "&#x2109;");
    } else {
      $("#cur_temp_label").html("<p color=red>error</p>");
    }
    $("#pump_label").html(data["pump"] == 0 ? "Off" : "On");
    $("#heater_label").html(data["heater"] == 0 ? "Off" : "On");
  });
}

function setTemp() {
  $.ajax({
    url: "/rest/state/set_temp",
    type: 'PUT',
    data: {"value": $("#new_temp").val()},
    contentType: 'application/json'
  });
}

function reboot() {
  $.ajax({
    url: "/reboot",
    type: 'PUT',
  });
}

function shutdown() {
  $.ajax({
    url: "/shutdown",
    type: 'PUT',
  });
}

$(function() {
  $.ajaxSetup({
      contentType : 'application/json',
      processData : false
  });
  $.ajaxPrefilter( function( options, originalOptions, jqXHR ) {
      if (options.data){
          options.data=JSON.stringify(options.data);
      }
  });

  $.getJSON("/rest/version", function (data) {
    $("#version").html("Version: " + data);
  });

  setInterval(refresh, 1000);

  $("#set_temp_button").click(setTemp);
  $("#reset_button").click(reset);
  $("#reboot").click(reboot);
  $("#shutdown").click(shutdown);
});
