subshader "pod_teemto_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.65 0.65 0.65;
	materialSpecularPower 12.5;
	envmap true;
	texture "texture/vehicles/PodRacer/pod_teemto_engine_id1";
}

subshader "pod_teemto_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "Mods\GCMOD\Movies\PodRacerCockpit";
}

