'use strict';

var app = angular.module('opening-hours', ['ngRoute', 'ngResource']);

app.config(['$routeProvider', function($routeProvider) {
    $routeProvider.when('/about', {
        templateUrl: 'partials/about.html'
    });

    $routeProvider.when('/:where?/:type?/:what?', {
        templateUrl: 'partials/search.html',
        controller: SearchController
    });
}]);

app.config(['$compileProvider', function($compileProvider) {
    $compileProvider.aHrefSanitizationWhitelist(/^\s*(https?|ftp|mailto|tel|file|geo):/);
}]);
