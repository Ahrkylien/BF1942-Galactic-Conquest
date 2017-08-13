subshader "runner_controls_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular false;
	envmap true;
	texture "texture/vehicles/runner/pilotcontrolscreens";
}

subshader "runner_controls_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular false;
	envmap true;
	texture "texture/vehicles/runner/frontpannel";
}

subshader "runner_controls_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
        transparent true;
	envmap true;
	texture "texture/vehicles/runner/overhang_screen";
}