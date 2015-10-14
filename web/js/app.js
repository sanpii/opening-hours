'use strict';

var app = angular.module('opening-hours', ['ngRoute', 'ngResource']);

app.config(['$routeProvider', function($routeProvider) {
    $routeProvider.when('/about', {
        templateUrl: 'partials/about.html'
    });

    $routeProvider.when('/:where?/:what?', {
        templateUrl: 'partials/search.html',
        controller: SearchController
    });
}]);
