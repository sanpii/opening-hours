function getState(value)
{
    var state = '';

    try {
        var opening  = new opening_hours(value.tags.opening_hours, {
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
                $scope.objects = [];

                response.data.elements.forEach(function (value) {
                    if (
                        typeof value.tags.name == 'undefined'
                        && typeof value.tags.amenity == 'undefined'
                    ) {
                        return;
                    }

                    $scope.objects.push({
                        name: value.tags.name,
                        amenity: value.tags.amenity,
                        opening_hours: value.tags.opening_hours,
                        state: getState(value),
                    });
                });
            });
        });
    };
}

SearchController.$inject = ['$scope', '$http'];
