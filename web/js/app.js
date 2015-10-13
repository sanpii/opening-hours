"use strict;"

document.addEventListener("DOMContentLoaded", function() {
    submit = document.getElementById("search");

    submit.addEventListener("click", search, true);
});

function search(event)
{
    where = document.getElementsByName("where");
    nomination(where[0].value);
    event.stopPropagation();
}

function nomination(name)
{
    url = 'http://nominatim.openstreetmap.org/search?format=json&q=' + name;

    http(url, function (data) {
        box = [
            data[0].boundingbox[0],
            data[0].boundingbox[2],
            data[0].boundingbox[1],
            data[0].boundingbox[3],
        ];
        updateBox(box);
    });
}

function updateBox(box)
{
    server = 'http://overpass-api.de/api/';
    url = server + '/interpreter?data=[out:json][timeout:25];node["opening_hours"](' + box.join() + ');out+body;';

    http(url, function (data) {
        updateList(data.elements);
    });
}

function updateList(objects)
{
    console.log(objects);

    element = document.getElementById("element");
    list = document.getElementById("list");

    objects.forEach(function (object) {
        ['amenity', 'name', 'opening_hours'].forEach(function (name) {
            child = element.content.querySelectorAll("." + name)[0];
            if (object.tags.hasOwnProperty(name)) {
                child.textContent = object.tags[name];
            }
            else {
                child.textContent = '¤';
            }
        });

        list.appendChild(element.content.cloneNode(true));
    });
}

function http(url, callback)
{
    var request = new XMLHttpRequest();
    request.onreadystatechange = function (event) {
        if (request.readyState == 4) {
            if (request.status == 200) {
                data = JSON.parse(request.response);
                callback(data);
            }
            else {
                // error
            }
        }
    };
    request.open('GET', url, false);
    request.send();
}
