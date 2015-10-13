function getState(node)
{
    var state = '';

    try {
        var opening  = new opening_hours(node.tags.opening_hours, {
            lat: location.lat,
            lon: location.lon,
            address: {
                country_code: 'fr',
            },
        });

        if (opening.getState()) {
            state = 'open';
        }
        else {
            state = 'closed';
        }
    }
    catch (e) {
    }

    return state;
}

function getIcon(node)
{
    var icon = '';
    var icons = {
        bicycle_parking: 'oc-parking-bicycle',
    };

    if (typeof node.tags.amenity !== 'undefined') {
        if (typeof icons[node.tags.amenity] !== 'undefined') {
            icon = icons[node.tags.amenity];
        }
        else {
            icon = 'oc-' + node.tags.amenity.replace('_', '-');
        }
    }

    return icon;
}

function SearchController($scope, $http)
{
    $scope.search = function () {
        $http({
            url: 'http://nominatim.openstreetmap.org/search?format=json&q=' + $scope.where
        }).then(function success(response) {
            var location = response.data[0];

            box = [
                location.boundingbox[0],
                location.boundingbox[2],
                location.boundingbox[1],
                location.boundingbox[3],
            ];

            $http({
                url: 'http://overpass-api.de/api/interpreter?data=[out:json][timeout:25];node["opening_hours"](' + box.join() + ');out+body;'
            }).then(function sucess(response) {
                $scope.nodes = [];

                response.data.elements.forEach(function (node) {
                    if (
                        typeof node.tags.name == 'undefined'
                        && typeof node.tags.amenity == 'undefined'
                    ) {
                        return;
                    }

                    $scope.nodes.push({
                        name: typeof node.tags.name !== 'undefined' ? node.tags.name : node.tags.amenity,
                        amenity: node.tags.amenity,
                        opening_hours: node.tags.opening_hours,
                        state: getState(node),
                        icon: getIcon(node),
                    });
                });
            });
        });
    };
}

SearchController.$inject = ['$scope', '$http'];
